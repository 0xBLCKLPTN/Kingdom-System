"use client"; // Указание, что это клиентский компонент

import { useEffect, useState } from "react";
import Image from "next/image";
import { MoonIcon, SunIcon, HomeIcon, UserGroupIcon, ServerIcon, CogIcon, XMarkIcon, ArchiveBoxIcon, ExclamationTriangleIcon, PhoneIcon } from '@heroicons/react/24/outline'; // Импортируем PhoneIcon
import { motion } from "framer-motion";
import ReactMarkdown from 'react-markdown'; // Импортируем react-markdown
import { useRouter } from 'next/navigation';
import styles from './Dashboard.module.css';
import { useSearchParams } from 'next/navigation'; 
import Cookie from "js-cookie";

const user = {
  name: "Имя Пользователя",
  avatar: "https://avatars.githubusercontent.com/u/73552143?v=4" // URL основного аватара
};

// Массив случайных пользователей
const friends = [
  { id: 1, name: "Друг 1", avatar: "https://i.pravatar.cc/300?img=1", online: true },
  { id: 2, name: "Друг 2", avatar: "https://i.pravatar.cc/300?img=2", online: false },
  { id: 3, name: "Друг 3", avatar: "https://i.pravatar.cc/300?img=3", online: true },
  { id: 4, name: "Друг 4", avatar: "https://i.pravatar.cc/300?img=4", online: false },
  { id: 5, name: "Друг 5", avatar: "https://i.pravatar.cc/300?img=5", online: true },
];

const Dashboard: React.FC = () => {
  const [isDarkMode, setIsDarkMode] = useState(false);
  const [animate, setAnimate] = useState(false);
  const [isFriendsPanelOpen, setFriendsPanelOpen] = useState(false);
  const [releases, setReleases] = useState<any[]>([]);
  const router = useRouter(); 
  const searchParams = useSearchParams(); // Получаем параметры URL
  const [username, setUsername] = useState<string>(Cookie.get("username") || ""); // Получаем имя пользователя из куков

  const toggleTheme = () => {
    setAnimate(true);
    setIsDarkMode(prev => !prev);
    setTimeout(() => setAnimate(false), 500);
  };

  const logoSrc = isDarkMode
    ? "https://raw.githubusercontent.com/0xBLCKLPTN/Kingdom-System/00fba0f093e419d8affffc5a797d24bbf8b1e0c3/Docs/illustrations/white/Kingdom-System.svg"
    : "https://raw.githubusercontent.com/0xBLCKLPTN/Kingdom-System/00fba0f093e419d8affffc5a797d24bbf8b1e0c3/Docs/illustrations/black/Kingdom-System.svg";

  const toggleFriendsPanel = () => {
    setFriendsPanelOpen(prev => !prev);
  };

  const handleCallsNavigation = () => {
    router.push('/calls'); // Перенаправление на страницу звонков
  };

  useEffect(() => {
    const fetchReleases = async () => {
      try {
        const response = await fetch('https://api.github.com/repos/0xBLCKLPTN/Kingdom-System/releases');
        const data = await response.json();
        setReleases(data);
      } catch (error) {
        console.error('Ошибка при получении релизов:', error);
      }
    };
    
    fetchReleases();
  }, []);

  return (
    <div className={`flex h-screen transition-colors duration-300 ${isDarkMode ? 'bg-gray-900' : 'bg-gray-100'} text-white`}>

      <motion.div 
        className={`absolute inset-0 ${isDarkMode ? 'bg-gray-900' : 'bg-gray-100'} transition-colors duration-500`} 
        initial={{ opacity: 0 }} 
        animate={animate ? { opacity: 1 } : { opacity: 0 }} 
        exit={{ opacity: 0 }} 
      />

      <aside className={`${styles.sidebar} ${isDarkMode ? 'bg-gray-800' : 'bg-white text-black'}`}>
        <div className={styles.logo}>
          <Image 
            src={logoSrc} 
            alt="Логотип Kingdom System"
            width={150}
            height={50}
            className="rounded-lg transition-transform duration-500 transform hover:scale-105"
          />
        </div>
        <nav className={styles.nav}>
          <motion.div whileHover={{ scale: 1.05 }} className={styles.navItem}>
            <HomeIcon className="w-6 h-6" />
            <span className={styles.itemText}>Главная</span>
          </motion.div>
          <motion.div whileHover={{ scale: 1.05 }} className={styles.navItem} onClick={toggleFriendsPanel}>
            <UserGroupIcon className="w-6 h-6" />
            <span className={styles.itemText}>Друзья</span>
          </motion.div>
          <motion.div whileHover={{ scale: 1.05 }} className={styles.navItem} onClick={handleCallsNavigation}> {/* Обработчик для перехода на страницу звонков */}
            <PhoneIcon className="w-6 h-6" />
            <span className={styles.itemText}>Звонки</span>
          </motion.div>
          {isFriendsPanelOpen && (
            <motion.div
              className={`${styles.friendsPanel} ${isDarkMode ? 'bg-gray-800 border-l border-white' : 'bg-white border-l border-gray-800'} p-4`}
              initial={{ opacity: 0, translateX: -20 }} 
              animate={{ opacity: 1, translateX: 0 }} 
              transition={{ duration: 0.3 }}
            >
              <ul className="mt-2">
                {friends.map(friend => (
                  <li key={friend.id} className="flex items-center my-2 mt-6">
                    <div className="relative">
                      <Image
                        src={friend.avatar}
                        alt={friend.name}
                        width={60}
                        height={60}
                        className="rounded-full"
                      />
                      {friend.online && 
                        <span className="absolute top-0 right-0 w-3 h-3 bg-green-500 border-2 border-white rounded-full"></span>
                      }
                    </div>
                    <span className="ml-2">{friend.name}</span>
                  </li>
                ))}
              </ul>
            </motion.div>
          )}
          <motion.div whileHover={{ scale: 1.05 }} className={styles.navItem}>
            <ServerIcon className="w-6 h-6" />
            <span className={styles.itemText}>Серверы</span>
          </motion.div>
          <motion.div whileHover={{ scale: 1.05 }} className={styles.navItem}>
            <CogIcon className="w-6 h-6" />
            <span className={styles.itemText}>Настройки</span>
          </motion.div>
          <motion.div whileHover={{ scale: 1.05 }} className={styles.navItem}>
            <ArchiveBoxIcon className="w-6 h-6" />
            <span className={styles.itemText}>Хранилище</span>
          </motion.div>
          <motion.div whileHover={{ scale: 1.05 }} className={styles.navItem} onClick={() => window.open('https://github.com/0xBLCKLPTN/Kingdom-System/issues/new', '_blank')}>
            <ExclamationTriangleIcon className="w-6 h-6" />
            <span className={styles.itemText}>Сообщить о баге</span>
          </motion.div>
        </nav>
        <div className={styles.userInfo}>
          <Image src={user.avatar} alt="Аватар" width={40} height={40} className="rounded-full" />
          <div className={styles.userDetails}>
            <p className={isDarkMode ? 'text-white' : 'text-black'}>{username }</p>
            <button className={`${styles.userButton} ${isDarkMode ? 'text-gray-400' : 'text-gray-600'}`}>
              <XMarkIcon className="w-4 h-4 inline-block mr-1" /> Выйти
            </button>
            <button className={`${styles.userButton} ${isDarkMode ? 'text-gray-400' : 'text-gray-600'}`}>
              <CogIcon className="w-4 h-4 inline-block mr-1" /> Настройки
            </button>
          </div>
        </div>
      </aside>
      <main className={styles.main} style={{ marginLeft: isFriendsPanelOpen ? '90px' : '0' }}>
        <header className={styles.header}>
          <div className={`${styles.headerContent} ${isDarkMode ? 'bg-gray-900' : 'bg-gray-300'} p-4 rounded-lg shadow-lg`}>
            <h1 className={isDarkMode ? 'text-white' : 'text-black'}>Добро пожаловать в Панель управления!</h1>
            <motion.button 
              onClick={toggleTheme}
              className={`p-2 rounded ${isDarkMode ? 'bg-gray-600 hover:bg-gray-500' : 'bg-gray-200 hover:bg-gray-300'} transition duration-200`}
              whileHover={{ scale: 1.1 }}
              whileTap={{ scale: 0.9 }}
            >
              {isDarkMode ? <MoonIcon className="w-6 h-6" /> : <SunIcon className="w-6 h-6" />}
            </motion.button>
          </div>
        </header>
        <motion.div 
          className={styles.content}
          initial={{ opacity: 0, translateY: 20 }} 
          animate={{ opacity: 1, translateY: 0 }} 
          transition={{ duration: 0.5 }}
        >

          {/* Отображение релизов в стиле GitHub с поддержкой Markdown */}
          <h2 className="mt-6 text-xl font-semibold">Релизы</h2>
          <div className="mt-2">
            {releases.map((release) => (
              <div key={release.id} className={`border rounded-lg p-4 mb-4 ${isDarkMode ? 'bg-gray-800 text-white' : 'bg-white text-black'}`}>
                <div className="flex justify-between items-center">
                  <h3 className={`text-lg font-bold ${isDarkMode ? 'text-white' : 'text-black'}`}>{release.name}</h3>
                  <span className={`text-sm ${release.prerelease ? 'text-yellow-500' : 'text-green-500'}`}>{release.prerelease ? 'Latest' : 'Stable'}</span>
                </div>
                <div className="mt-2">
                  <ReactMarkdown className={`prose ${isDarkMode ? 'prose-invert' : 'prose'}`}>
                    {release.body}
                  </ReactMarkdown>
                </div>
                <div className="mt-2">
                  <a href={release.html_url} className="text-blue-500" target="_blank" rel="noopener noreferrer">Полный журнал изменений</a>
                </div>
                {release.assets.length > 0 && (
                  <div className="mt-4">
                    <h4 className={`font-semibold ${isDarkMode ? 'text-white' : 'text-black'}`}>Больше</h4>
                    <a href="https://github.com/0xBLCKLPTN/Kingdom-System/releases" className="text-blue-500">https://github.com/0xBLCKLPTN/Kingdom-System/releases</a>
                  </div>
                )}
              </div>
            ))}
          </div>
        </motion.div>
      </main>
    </div>
  );
};

export default Dashboard;
