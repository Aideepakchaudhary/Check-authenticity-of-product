# Check Authenticity
Only manufacturing unit will allow to add all the hash of the product.

 Check if hash is found in the storage that means the product is authentic.

 Should have mechanism to add manufacturing unit

 Anybody can see the barcode on the product and try to copy that barcode and put on the fake product?

### Conditions
```
How to generate a unique hash?
What are the details required to add the manufacturer?
How to prevent qr copy?
```

### Corner cases
```
1. We Need to store the products in a different storage(partially_selled_products) which is scanned first..so that no one can copy the product QR.
2. If a user wants to return the Product and we have 15 days for it.until then this product is stored in partially_selled_products.
3. After the return time is over the product goes to sell_product.
4. If product is returned under the given time slot.
5. If a item is return then it can't be sale now.
6. To sale a returned item, item goes to the manufacturing unit and they check their originality.
7. If the confirmation comes from manufacturing unit, the amount of the customer will be released.
```
