function generateRecommendations(
  viewedProducts: any[],
  allProducts: any[]
): any[] {
  const weightMap: Record<string, number> = {};

  // Assign weights based on recency and frequency
  viewedProducts.forEach((product, index) => {
    const weight = viewedProducts.length - index; // Recent products have higher weights
    weightMap[product.category] = (weightMap[product.category] || 0) + weight;

    product.tags.forEach((tag: string) => {
      weightMap[tag] = (weightMap[tag] || 0) + weight;
    });
  });

  // Score all products based on the weight map
  const scoredProducts = allProducts.map((product) => {
    let score = 0;

    if (weightMap[product.category]) score += weightMap[product.category];
    product.tags.forEach((tag: string) => {
      if (weightMap[tag]) score += weightMap[tag];
    });

    return { product, score };
  });

  // Sort by score and return the top products
  return scoredProducts
    .sort((a, b) => b.score - a.score)
    .map((item) => item.product)
    .slice(0, 10);
}
