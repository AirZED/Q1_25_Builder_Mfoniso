import bs58 from "bs58";

export const base58_to_wallet = (base58_str: string) => {
  return bs58.decode(base58_str);
};

export const wallet_to_base58 = (wallet: Uint8Array | number[]) => {
  return bs58.encode(wallet);
};

// tests
console.log(
  wallet_to_base58([
    34, 46, 55, 124, 141, 190, 24, 204, 134, 91, 70, 184, 161, 181, 44, 122, 15,
    172, 63, 62, 153, 150, 99, 255, 202, 89, 105, 77, 41, 89, 253, 130, 27, 195,
    134, 14, 66, 75, 242, 7, 132, 234, 160, 203, 109, 195, 116, 251, 144, 44,
    28, 56, 231, 114, 50, 131, 185, 168, 138, 61, 35, 98, 78, 53,
  ])
);

console.log(
  base58_to_wallet(
    "gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VvksUGhPt4SZ8JHVSkt"
  )
);
