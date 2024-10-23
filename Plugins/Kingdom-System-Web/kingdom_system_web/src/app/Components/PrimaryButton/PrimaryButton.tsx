import { motion } from "framer-motion";

export default function PrimaryButton(text: string)
{
    return (
        <div
          className="w-full p-3 bg-blue-500 text-white rounded hover:bg-blue-600 transition duration-200"
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
        >
          Войти
        </div>
    )
}