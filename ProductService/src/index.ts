import express from "express";
import dotenv from "dotenv";
import connectDB from "./db";
import productRoutes from "./product.routes";
import errorHandler from "./product.errorHandler";

dotenv.config();

const app = express();

app.use(express.json());

connectDB();

app.use("/api/products", productRoutes);
app.use(errorHandler);

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
});
