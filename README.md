Based on https://github.com/dcomartin/LooselyCoupledMonolith

Uses registry pattern from  https://willcrichton.net/rust-api-type-patterns/registries.html to allow loose-coupling.

Currently POST `http://localhost:8000/sales` will dispatch a `PlacedOrder` event which will be handled by the registered handler `create_shipping_label`.
Also have added some service state used in handler, as it makes it a little more real-world on how event handlers would have to be registered.
