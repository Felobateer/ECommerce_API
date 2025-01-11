import mongoose from "mongoose";

const connectDB = async () => {
  try {
    const uri = process.env.MONGO_URI || "mongodb://localhost:27017/products";
    await mongoose.connect(uri);
    console.log("MongoDB connected!");
  } catch (error) {
    console.error("Error connecting to the database: " + error);
    process.exit(1);
  }
};

export default connectDB;
