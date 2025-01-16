ECommerce API

This API is structured as microservices, but all the services are contained in a single repository for convenience. The API is built using `Express.js` (with TypeScript) and `Rust` leveraging the `Actix` and `Axum` frameworks. The main components of the API include the `User`, `Product`, and `Cart` services, along with a `Payment` service and a `Recommendation Algorithm`.

Features 1. User registration and login. 2. Product addition and display. 3. Recommendation algorithms that use user cookies to suggest products the user is likely to be interested in. 4. A secure payment gateway. 5. Add-to-cart functionality.
