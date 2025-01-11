import { Router, Request, Response } from "express";
import authorize from "./product.middleware";
import validateRequest from "./product.validateRequest";
import productSchema from "./product.validation";
import {
  createProduct,
  getAllProducts,
  getProductById,
  updateProduct,
  deleteProduct,
} from "./product.service";

const router = Router();

// Apply authorization middleware globally to the routes
router.use(authorize);

// Create a product
router.post(
  "/",
  validateRequest(productSchema),
  async (req: Request, res: Response) => {
    try {
      const product = await createProduct(req.body);
      res.status(201).json(product);
    } catch (error) {
      res.status(400).json({ error: error.message });
    }
  }
);

// Get all products
router.get("/", async (req: Request, res: Response) => {
  try {
    const products = await getAllProducts();
    res.json(products);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get a product by ID
router.get("/:id", async (req: Request, res: Response) => {
  try {
    const product = await getProductById(req.params.id);
    if (!product) {
      return res.status(404).json({ message: "Product not found" });
    }
    res.json(product);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Update a product by ID
router.put(
  "/:id",
  validateRequest(productSchema),
  async (req: Request, res: Response) => {
    try {
      const product = await updateProduct(req.params.id, req.body);
      if (!product) {
        return res.status(404).json({ message: "Product not found" });
      }
      res.json(product);
    } catch (error) {
      res.status(400).json({ error: error.message });
    }
  }
);

// Delete a product by ID
router.delete("/:id", async (req: Request, res: Response) => {
  try {
    const product = await deleteProduct(req.params.id);
    if (!product) {
      return res.status(404).json({ message: "Product not found" });
    }
    res.json({ message: "Product deleted successfully" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

export default router;
