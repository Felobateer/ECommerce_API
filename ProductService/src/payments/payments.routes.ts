import express, { Router, Request, Response, NextFunction } from "express";
import { createPaymentIntent, retrievePaymentIntent } from "./payment.service";
import Stripe from "stripe";

const router = Router();

const stripe = new Stripe(process.env.STRIPE_SECRET_KEY as string, {
  apiVersion: "2024-12-18.acacia",
});

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

// Handle webhook events
router.post(
  "/webhook",
  express.raw({ type: "application/json" }), // Ensure the body is raw for Stripe to verify signatures
  async (req: Request, res: Response): Promise<void> => {
    // Fixed the function signature
    const sig = req.headers["stripe-signature"] as string | undefined;
    const endpointSecret = process.env.STRIPE_WEBHOOK_SECRET as string;

    if (!sig || !endpointSecret) {
      console.error("Missing Stripe signature or webhook secret");
      res.status(400).send("Webhook signature or secret missing");
      return;
    }

    let event: Stripe.Event;

    try {
      // Verify the webhook signature
      event = stripe.webhooks.constructEvent(req.body, sig, endpointSecret);
    } catch (err: any) {
      console.error(`Webhook Error: ${err.message}`);
      res.status(400).send(`Webhook Error: ${err.message}`);
      return;
    }

    // Handle the event based on its type
    switch (event.type) {
      case "payment_intent.succeeded": {
        const paymentIntent = event.data.object as Stripe.PaymentIntent;
        console.log(`PaymentIntent succeeded: ${paymentIntent.id}`);
        break;
      }
      case "payment_intent.payment_failed": {
        const failedPaymentIntent = event.data.object as Stripe.PaymentIntent;
        console.error(`PaymentIntent failed: ${failedPaymentIntent.id}`);
        break;
      }
      default:
        console.warn(`Unhandled event type: ${event.type}`);
    }

    // Respond to Stripe to confirm receipt of the event
    res.status(200).send("Webhook received");
  }
);

export default router;
