"use client";

import { useState } from "react";
import Image from "next/image";
import { MoonIcon, SunIcon } from '@heroicons/react/24/outline'; 
import { motion } from "framer-motion";
import { useRouter } from 'next/navigation';

const GITHUB_OAUTH_URL = "https://github.com/login/oauth/authorize"; // Указание URL для авторизации

export default function Home() {
  const [isDarkMode, setIsDarkMode] = useState(false);
  const router = useRouter();

  const toggleTheme = () => {
    setIsDarkMode(!isDarkMode);
  };

  const imageSrc = isDarkMode
    ? "https://raw.githubusercontent.com/0xBLCKLPTN/Kingdom-System/00fba0f093e419d8affffc5a797d24bbf8b1e0c3/Docs/illustrations/white/Kingdom-System.svg"
    : "https://raw.githubusercontent.com/0xBLCKLPTN/Kingdom-System/00fba0f093e419d8affffc5a797d24bbf8b1e0c3/Docs/illustrations/black/Kingdom-System.svg";

  const handleLogin = () => {
    router.push('/dashboard');
  };

  const handleGitHubLogin = () => {
    const clientId = "YOUR_GITHUB_CLIENT_ID"; // Замените на ваш клиентский ID
    const redirectUri = encodeURIComponent("http://localhost:3000/auth/github/callback"); // Ваш URL перенаправления
    const scope = "read:user user:email"; // Запрашиваемые разрешения

    window.location.href = `${GITHUB_OAUTH_URL}?client_id=${clientId}&redirect_uri=${redirectUri}&scope=${scope}`;
  };

  return (
    <div className={`flex flex-col items-center justify-center h-screen p-4 transition-colors duration-300 ${isDarkMode ? 'bg-gray-900' : 'bg-white'}`}>
      <motion.div
        className="relative"
        initial={{ y: -50, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ duration: 0.5 }}
      >
        <Image 
          alt="logo" 
          src={imageSrc} 
          width="300" 
          height="100" 
          className="rounded-lg transition-transform duration-500 transform hover:scale-105"
        />
      </motion.div>

      <motion.div
        className="w-full max-w-sm mt-5"
        initial={{ scale: 0 }}
        animate={{ scale: 1 }}
        transition={{ duration: 0.5 }}
      >
        <input
          type="text"
          placeholder="Email"
          className={`w-full p-3 border border-gray-300 rounded mb-3 transition-all duration-300 ${isDarkMode ? 'bg-gray-800 text-white' : ''} focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        <input
          type="password"
          placeholder="Password"
          className={`w-full p-3 border border-gray-300 rounded mb-3 transition-all duration-300 ${isDarkMode ? 'bg-gray-800 text-white' : ''} focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        
        {/* Кнопка для входа */}
        <motion.button 
          onClick={handleLogin}
          className="w-full p-3 bg-blue-500 text-white rounded hover:bg-blue-600 transition duration-200"
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
        >
          Войти
        </motion.button>

        {/* Кнопка для входа через GitHub */}
        <motion.button 
          onClick={handleGitHubLogin} // Обработка логина через GitHub
          className="w-full flex items-center justify-center p-3 mt-2 bg-gray-300 text-gray-700 rounded hover:bg-gray-400 transition duration-200"
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
        >
          <svg className="w-5 h-5 mr-2" aria-hidden="true" fill="currentColor" viewBox="0 0 16 16">
            <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.54 5.47 7.61.4.07.55-.17.55-.37 0-.19-.01-.72-.01-1.41-2.24.49-2.71-1.06-2.71-1.06-.36-.91-.88-1.15-.88-1.15-.72-.49.06-.48.06-.48.79.06 1.2.81 1.2.81.7 1.2 1.84.85 2.29.65.07-.51.27-.85.49-1.05-1.74-.2-3.56-.87-3.56-3.85 0-.85.3-1.55.79-2.1-.08-.2-.34-1.01.07-2.1 0 0 .66-.21 2.15.81.62-.17 1.29-.26 1.95-.26.66 0 1.34.09 1.96.26 1.49-1.02 2.15-.81 2.15-.81.41 1.09.14 1.9.07 2.1.49.55.79 1.25.79 2.1 0 2.98-1.83 3.65-3.57 3.85.28.24.52.72.52 1.46 0 1.05-.01 1.9-.01 2.16 0 .2.15.44.55.37C13.71 14.54 16 11.54 16 8c0-4.42-3.58-8-8-8z"/>
          </svg>
          Войти используя GitHub
        </motion.button>

        <motion.button 
          className="w-full p-3 bg-gray-300 text-gray-700 rounded mt-2 hover:bg-gray-400 transition duration-200"
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
        >
          Регистрация
        </motion.button>

        {/* Картинка кота */}
        <Image 
          src="https://github.com/0xBLCKLPTN/Kingdom-System/blob/main/Docs/illustrations/Deve.png?raw=true" 
          alt="Котик" 
          width={200} 
          height={200} 
          className="ml-20 mt-20"
        />
      </motion.div>

      <motion.button 
        onClick={toggleTheme}
        className={`absolute bottom-4 right-4 p-2 rounded ${isDarkMode ? 'bg-gray-600 text-white' : 'bg-gray-200 text-black'}`}
        whileHover={{ scale: 1.1 }}
        whileTap={{ scale: 0.9 }}
      >
        {isDarkMode ? 
          <MoonIcon className="w-6 h-6" /> : 
          <SunIcon className="w-6 h-6" />
        }
      </motion.button>
    </div>
  );
}
