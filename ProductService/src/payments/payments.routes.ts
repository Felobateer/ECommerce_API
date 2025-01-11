import { Router, Request, Response, NextFunction } from "express";
import { createPaymentIntent, retrievePaymentIntent } from "./payment.service";

const router = Router();

// Create a payment intent
router.post(
  "/create-payment-intent",
  async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    try {
      const { amount, currency, metadata } = req.body;

      if (!amount || !currency) {
        res.status(400).json({ error: "Amount and currency are required." });
        return; // Ensure the function ends here
      }

      const paymentIntent = await createPaymentIntent(
        amount,
        currency,
        metadata
      );
      res.status(201).json({ clientSecret: paymentIntent.client_secret });
    } catch (error) {
      next(error); // Pass the error to the error-handling middleware
    }
  }
);

// Retrieve a payment intent
router.get(
  "/payment-intent/:id",
  async (req: Request, res: Response, next: NextFunction) => {
    try {
      const { id } = req.params;

      const paymentIntent = await retrievePaymentIntent(id);
      res.status(200).json(paymentIntent);
    } catch (error) {
      next(error);
    }
  }
);

export default router;
