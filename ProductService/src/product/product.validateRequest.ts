import { Response, Request, NextFunction } from "express";
import { ObjectSchema } from "joi";

// Middleware factory to validate requests based on a Joi schema
const validateRequest =
  (schema: ObjectSchema) =>
  (req: Request, res: Response, next: NextFunction): void => {
    const { error } = schema.validate(req.body, { abortEarly: false });
    if (error) {
      res.status(400).json({
        success: false,
        errors: error.details.map((detail) => detail.message),
      });
      return; // Explicitly return to prevent further execution
    }
    next(); // Pass control to the next middleware
  };

export default validateRequest;
