import express from "express";
import dotenv from "dotenv";
import connectDB from "./db";
import productRoutes from "./product/product.routes";
import paymentsRoutes from "./payments/payments.routes";
import errorHandler from "./product/product.errorHandler";
import cors from "cors";
import cookieParser from "cookie-parser";

dotenv.config();

const app = express();

app.use(express.json());
app.use(cors({ origin: "*" }));
app.use(cookieParser());

connectDB();

app.use("/api/products", productRoutes);
app.use("/api/payment", paymentsRoutes);
app.use(errorHandler);

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
});
