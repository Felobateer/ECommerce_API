import { Request, Response, NextFunction } from "express";

// Centralized error handling middleware
const errorHandler = (
  err: any,
  req: Request,
  res: Response,
  next: NextFunction
) => {
  console.error(err); // Log the error for debugging

  // Extract the error details
  const statusCode = err.statusCode || 500;
  const message = err.message || "Internal Server Error";

  // Send a consistent error response
  res.status(statusCode).json({
    success: false,
    error: message,
  });
};

export default errorHandler;
