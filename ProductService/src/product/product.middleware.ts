import { Request, Response, NextFunction, RequestHandler } from "express";

const authorize: RequestHandler = (
  req: Request,
  res: Response,
  next: NextFunction
): void => {
  const apiKey = req.headers["x-api-key"];

  // Check if API key is missing
  if (!apiKey) {
    res.status(401).json({
      success: false,
      error: "Unauthorized: API key is required.",
    });
    return; // Explicitly return to end the function after sending the response
  }

  // Check if the API key is valid
  const validApiKeys = process.env.VALID_API_KEYS?.split(",") || [];
  if (!validApiKeys.includes(apiKey as string)) {
    res.status(403).json({
      success: false,
      error: "Forbidden: Invalid API key.",
    });
    return; // Explicitly return to end the function after sending the response
  }

  // If API key is valid, proceed to the next middleware or route handler
  next();
};

export default authorize;
