import Joi from "joi";

// Joi schema for product creation validation
const productSchema = Joi.object({
  name: Joi.string().required().messages({
    "string.empty": "Name is required.",
  }),
  description: Joi.string().required(),
  price: Joi.number().min(0).required(),
  category: Joi.string().required(),
  quantity: Joi.number().min(0).required(),
  notes: Joi.string().optional(),
});

export default productSchema;
