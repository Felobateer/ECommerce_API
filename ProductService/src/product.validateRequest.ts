import { Response, Request, NextFunction, RequestHandler } from "express";
import { ObjectSchema } from "joi";

// Middleware to validate requests based on a Joi schema
const validateRequest: RequestHandler = (schema: ObjectSchema) => {
  return (req: Request, res: Response, next: NextFunction): void => {
    const { error } = schema.validate(req.body, { abortEarly: false });
    if (error) {
      return res.status(400).json({
        success: false,
        errors: error.details.map((detail) => detail.message),
      });
    }
    next();
  };
};

export default validateRequest;
