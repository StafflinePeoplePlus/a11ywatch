import { detectImageModel } from "@app/core/ai";

const detectImage = async (req, res, next) => {
  try {
    const data = await detectImageModel({
      img: req.body.img,
    });

    res.json(data);
  } catch (e) {
    console.log(`Error: `, e);
    next();
  }
};

export { detectImage };
