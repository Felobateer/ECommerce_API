import { Router, Request, Response, NextFunction } from "express";
import authorize from "./product.middleware"; // Authorization middleware
import validateRequest from "./product.validateRequest"; // Validation middleware
import productSchema from "./product.validation"; // Joi validation schema
import {
  createProduct,
  getAllProducts,
  getProductById,
  updateProduct,
  deleteProduct,
} from "./product.service";

// Helper to wrap async handlers
const asyncHandler =
  (fn: (req: Request, res: Response, next: NextFunction) => Promise<void>) =>
  (req: Request, res: Response, next: NextFunction) => {
    fn(req, res, next).catch(next);
  };

const router = Router();

// Apply authorization middleware globally to the routes
router.use((req: Request, res: Response, next: NextFunction) => {
  authorize(req, res, next);
});

// Create a product
router.post(
  "/",
  (req: Request, res: Response, next: NextFunction) => {
    validateRequest(productSchema)(req, res, next);
  },
  async (req: Request, res: Response) => {
    try {
      const product = await createProduct(req.body);
      res.status(201).json(product);
    } catch (error: any) {
      res.status(400).json({ error: error.message });
    }
  }
);

// Get all products
router.get("/", async (req: Request, res: Response) => {
  try {
    const products = await getAllProducts();
    res.json(products);
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Get a product by ID
router.get(
  "/:id",
  asyncHandler(async (req: Request, res: Response) => {
    const product = await getProductById(req.params.id);
    if (!product) {
      res.status(404).json({ message: "Product not found" });
      return;
    }
    res.json(product);
  })
);

// Update a product by ID
router.put(
  "/:id",
  validateRequest(productSchema),
  asyncHandler(async (req: Request, res: Response) => {
    const product = await updateProduct(req.params.id, req.body);
    if (!product) {
      res.status(404).json({ message: "Product not found" });
      return;
    }
    res.json(product);
  })
);

// Delete a product by ID
router.delete(
  "/:id",
  asyncHandler(async (req: Request, res: Response) => {
    const product = await deleteProduct(req.params.id);
    if (!product) {
      res.status(404).json({ message: "Product not found" });
      return;
    }
    res.json({ message: "Product deleted successfully" });
  })
);

export default router;
