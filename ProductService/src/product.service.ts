import Product, { IProduct } from "./product.model";

// Create a new product
export const createProduct = async (data: IProduct) => {
  const product = new Product(data);
  return await product.save();
};

// Get all products
export const getAllProducts = async () => {
  return await Product.find();
};

// Get a single product by ID
export const getProductById = async (id: string) => {
  return await Product.findById(id);
};

// Update a product by ID
export const updateProduct = async (id: string, data: Partial<IProduct>) => {
  return await Product.findByIdAndUpdate(id, data, { new: true });
};

// Delete a product by ID
export const deleteProduct = async (id: string) => {
  return await Product.findByIdAndDelete(id);
};
