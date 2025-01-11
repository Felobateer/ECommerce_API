import Stripe from "stripe";
import dotenv from "dotenv";

const stripe = new Stripe(process.env.STRIPE_SECRET_KEY as string, {
  apiVersion: "2024-12-18.acacia",
});

export const createPaymentIntent = async (
  amount: number,
  currency: string,
  metadata?: Record<string, string>
): Promise<Stripe.PaymentIntent> => {
  return await stripe.paymentIntents.create({
    amount,
    currency,
    metadata,
  });
};

export const retrievePaymentIntent = async (
  paymentIntentId: string
): Promise<Stripe.PaymentIntent> => {
  return await stripe.paymentIntents.retrieve(paymentIntentId);
};
