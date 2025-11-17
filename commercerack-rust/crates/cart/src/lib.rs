use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a single item in the shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    pub sku: String,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: Decimal,
}

impl CartItem {
    pub fn new(sku: String, product_name: String, quantity: i32, unit_price: Decimal) -> Self {
        Self {
            sku,
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn subtotal(&self) -> Decimal {
        self.unit_price * Decimal::from(self.quantity)
    }
}

/// Shopping cart with in-memory storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub cart_id: String,
    pub items: Vec<CartItem>,
}

impl Cart {
    /// Create a new empty cart with a unique ID
    pub fn new() -> Self {
        Self {
            cart_id: Uuid::new_v4().to_string(),
            items: Vec::new(),
        }
    }

    /// Create a cart with a specific ID (for restoration)
    pub fn with_id(cart_id: String) -> Self {
        Self {
            cart_id,
            items: Vec::new(),
        }
    }

    /// Add an item to the cart. If SKU already exists, increase quantity
    pub fn add_item(&mut self, sku: String, product_name: String, quantity: i32, unit_price: Decimal) {
        if let Some(existing) = self.items.iter_mut().find(|item| item.sku == sku) {
            existing.quantity += quantity;
        } else {
            self.items.push(CartItem::new(sku, product_name, quantity, unit_price));
        }
    }

    /// Remove an item completely from the cart
    pub fn remove_item(&mut self, sku: &str) -> bool {
        if let Some(pos) = self.items.iter().position(|item| item.sku == sku) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    /// Update quantity for a specific SKU. Returns false if SKU not found
    pub fn update_quantity(&mut self, sku: &str, new_quantity: i32) -> bool {
        if new_quantity <= 0 {
            return self.remove_item(sku);
        }

        if let Some(item) = self.items.iter_mut().find(|item| item.sku == sku) {
            item.quantity = new_quantity;
            true
        } else {
            false
        }
    }

    /// Get item by SKU
    pub fn get_item(&self, sku: &str) -> Option<&CartItem> {
        self.items.iter().find(|item| item.sku == sku)
    }

    /// Calculate cart subtotal (sum of all item subtotals)
    pub fn subtotal(&self) -> Decimal {
        self.items.iter().map(|item| item.subtotal()).sum()
    }

    /// Get total item count in cart
    pub fn item_count(&self) -> i32 {
        self.items.iter().map(|item| item.quantity).sum()
    }

    /// Clear all items from cart
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Check if cart is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for Cart {
    fn default() -> Self {
        Self::new()
    }
}

/// Cart store for managing multiple carts (in-memory storage)
/// In production, this would be backed by Redis or a database
pub struct CartStore {
    carts: HashMap<String, Cart>,
}

impl CartStore {
    pub fn new() -> Self {
        Self {
            carts: HashMap::new(),
        }
    }

    pub fn create_cart(&mut self) -> String {
        let cart = Cart::new();
        let cart_id = cart.cart_id.clone();
        self.carts.insert(cart_id.clone(), cart);
        cart_id
    }

    pub fn get_cart(&self, cart_id: &str) -> Option<&Cart> {
        self.carts.get(cart_id)
    }

    pub fn get_cart_mut(&mut self, cart_id: &str) -> Option<&mut Cart> {
        self.carts.get_mut(cart_id)
    }

    pub fn save_cart(&mut self, cart: Cart) {
        self.carts.insert(cart.cart_id.clone(), cart);
    }

    pub fn delete_cart(&mut self, cart_id: &str) -> bool {
        self.carts.remove(cart_id).is_some()
    }
}

impl Default for CartStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cart_operations() {
        let mut cart = Cart::new();
        assert!(cart.is_empty());
        assert_eq!(cart.item_count(), 0);

        // Add items
        cart.add_item(
            "SKU001".to_string(),
            "Widget".to_string(),
            2,
            Decimal::new(1999, 2), // $19.99
        );
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.item_count(), 2);
        assert_eq!(cart.subtotal(), Decimal::new(3998, 2)); // $39.98

        // Add same SKU again (should merge)
        cart.add_item(
            "SKU001".to_string(),
            "Widget".to_string(),
            3,
            Decimal::new(1999, 2),
        );
        assert_eq!(cart.items.len(), 1); // Still 1 unique item
        assert_eq!(cart.item_count(), 5); // 2 + 3 = 5 total
        assert_eq!(cart.subtotal(), Decimal::new(9995, 2)); // $99.95

        // Add different SKU
        cart.add_item(
            "SKU002".to_string(),
            "Gadget".to_string(),
            1,
            Decimal::new(2999, 2), // $29.99
        );
        assert_eq!(cart.items.len(), 2);
        assert_eq!(cart.item_count(), 6);
        assert_eq!(cart.subtotal(), Decimal::new(12994, 2)); // $129.94
    }

    #[test]
    fn test_cart_update_and_remove() {
        let mut cart = Cart::new();

        cart.add_item("SKU001".to_string(), "Widget".to_string(), 5, Decimal::new(1000, 2));
        cart.add_item("SKU002".to_string(), "Gadget".to_string(), 3, Decimal::new(2000, 2));

        // Update quantity
        assert!(cart.update_quantity("SKU001", 10));
        assert_eq!(cart.get_item("SKU001").unwrap().quantity, 10);

        // Update to zero should remove
        assert!(cart.update_quantity("SKU001", 0));
        assert_eq!(cart.items.len(), 1);
        assert!(cart.get_item("SKU001").is_none());

        // Remove item
        assert!(cart.remove_item("SKU002"));
        assert!(cart.is_empty());
        assert_eq!(cart.subtotal(), Decimal::ZERO);
    }

    #[test]
    fn test_cart_store() {
        let mut store = CartStore::new();

        // Create cart
        let cart_id = store.create_cart();
        assert!(store.get_cart(&cart_id).is_some());

        // Add items to cart
        if let Some(cart) = store.get_cart_mut(&cart_id) {
            cart.add_item("SKU001".to_string(), "Widget".to_string(), 2, Decimal::new(1999, 2));
        }

        // Verify cart has item
        let cart = store.get_cart(&cart_id).unwrap();
        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.item_count(), 2);

        // Delete cart
        assert!(store.delete_cart(&cart_id));
        assert!(store.get_cart(&cart_id).is_none());
    }
}
