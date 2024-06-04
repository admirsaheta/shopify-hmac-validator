## Shopify HMAC validator

To validate your HMAC ( and for safety reasons ) please clone the app and download onto your local machine
Build the project using ```cargo build``` and then run it with help of ```cargo run```

After running the app pass your shopify app secret in the valid field, your query params and generated HMAC.

The script will validate if the HMAC is correct.
