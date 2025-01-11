import { Router, Request, Response, NextFunction } from "express";
import authorize from "./product.middleware"; // Authorization middleware
import validateRequest from "./product.validateRequest"; // Validation middleware
import productSchema from "./product.validation"; // Joi validation schema
import Product, { IProduct } from "./product.model";
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

// Get recommended products
router.get(
  "/recommendation",
  asyncHandler(async (req: Request, res: Response): Promise<void> => {
    // Fetch all products from the database
    const products: IProduct[] = await Product.find().lean(); // Adjusting to proper database query

    // Get the viewed products from cookies
    const viewedProducts: { category: string; tags: string[] }[] = req.cookies
      .viewedProducts
      ? JSON.parse(req.cookies.viewedProducts)
      : [];

    // If no products were viewed, return an empty list of recommendations
    if (!viewedProducts.length) {
      res.status(200).send({ recommendations: [] });
      return;
    }

    // Extract categories and tags from viewed products
    const categories = viewedProducts.map((p) => p.category);
    const tags = viewedProducts.flatMap((p) => p.tags);

    // Query for related products based on categories or tags
    const recommendations: IProduct[] = await Product.find({
      $or: [{ category: { $in: categories } }, { tags: { $in: tags } }],
    })
      .limit(10) // Limit to 10 recommendations
      .lean();

    // Send the recommended products
    res.status(200).send({ recommendations });
  })
);

// Get a product by ID
router.get(
  "/:id",
  asyncHandler(async (req: Request, res: Response) => {
    const productId = req.params.id;
    const { category, tags } = req.query; // Assume category and tags are passed

    const viewedProducts = req.cookies.viewedProducts
      ? JSON.parse(req.cookies.viewedProducts)
      : [];

    // Add the new product to the cookie (limit to 10 entries)
    const updatedViewedProducts = [
      ...new Set([...viewedProducts, { productId, category, tags }]),
    ].slice(-10);

    res.cookie("viewedProducts", JSON.stringify(updatedViewedProducts), {
      httpOnly: true,
      maxAge: 7 * 24 * 60 * 60 * 1000, // 7 days
    });

    res.status(200).send({ message: "Product viewed", updatedViewedProducts });
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
