import { Request, Response, NextFunction, RequestHandler } from "express";

// Middleware for API key-based authorization
const authorize: RequestHandler = (
  req: Request,
  res: Response,
  next: NextFunction
) => {
  const apiKey = req.headers["x-api-key"];
  if (!apiKey) {
    return res.status(401).json({
      success: false,
      error: "Unauthorized: API key is required.",
    });
  }

  const validApiKeys = process.env.VALID_API_KEYS?.split(",") || [];
  if (!validApiKeys.includes(apiKey as string)) {
    return res.status(403).json({
      success: false,
      error: "Forbidden: Invalid API key.",
    });
  }
  next();
};

export default authorize;
