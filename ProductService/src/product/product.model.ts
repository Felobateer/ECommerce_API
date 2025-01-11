import mongoose, { Schema, Document } from "mongoose";
import Joi from "joi";

export interface IProduct extends Document {
  name: string;
  description: string;
  notes?: string;
  price: number;
  category: string;
  quantity: number;
  createdAt?: Date;
  updatedAt?: Date;
}

// Create the Product schema
const ProductSchema: Schema = new Schema(
  {
    name: {
      type: String,
      required: [true, "Product name is required"],
      trim: true,
    },
    description: {
      type: String,
      required: true,
      trim: true,
    },
    notes: {
      type: String,
      required: false,
      trim: true,
    },
    price: {
      type: Number,
      required: [true, "Price is required"],
      min: [0, "Price must be a positive number"],
    },
    category: {
      type: String,
      required: true,
    },
    quantity: {
      type: Number,
      required: [true, "Quantity is required"],
      min: [0, "Quantity must be a positive number"],
    },
  },
  {
    timestamps: true, // Automatically add createdAt and updatedAt fields
  }
);

// Create the Product model
const Product = mongoose.model<IProduct>("Product", ProductSchema);

export default Product;
