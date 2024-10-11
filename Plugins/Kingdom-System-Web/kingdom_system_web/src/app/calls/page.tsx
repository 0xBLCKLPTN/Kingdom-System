"use client";

import { useEffect, useState } from "react";
import Cookie from "js-cookie"; 
import { firestore } from "../firebase"; 
import { collection, doc, setDoc, onSnapshot, getDoc, updateDoc, deleteDoc } from "firebase/firestore"; 
import { motion } from "framer-motion";
import { useRouter } from 'next/navigation'; // Импортируем useRouter

const colors = [
    "#FF5733", "#33FF57", "#3357FF", "#FF33A6", "#33FFA6", 
    "#FFD433", "#FF3333", "#33FF33", "#3333FF", "#A633FF"
];

const getColor = (index: number) => {
    return colors[index % colors.length]; 
};

const Calls: React.FC = () => {
    const [roomName, setRoomName] = useState<string>(''); 
    const [username, setUsername] = useState<string>(''); 
    const [rooms, setRooms] = useState<string[]>([]);
    const [currentUsers, setCurrentUsers] = useState<string[]>([]);
    const [currentUserId, setCurrentUserId] = useState<string>(''); 
    const [creatorId, setCreatorId] = useState<string>(''); 
    const [showCreateRoom, setShowCreateRoom] = useState<boolean>(false); 
    const [showUsernamePrompt, setShowUsernamePrompt] = useState<boolean>(true); 

    const router = useRouter(); // Инициализация useRouter

    // Загрузка имени пользователя из куков
    useEffect(() => {
        const cookieUsername = Cookie.get('username');
        if (cookieUsername) {
            setUsername(cookieUsername);
            setCurrentUserId(cookieUsername);
            setShowUsernamePrompt(false);
        }
    }, []);

    // Создание комнаты
    const createRoom = async () => {
        if (roomName) {
            try {
                await setDoc(doc(firestore, 'rooms', roomName), {
                    id: roomName,
                    users: [],
                    creator: currentUserId 
                });
                joinRoom(roomName); 
            } catch (error) {
                console.error("Ошибка при создании комнаты: ", error);
            }
            setShowCreateRoom(false);
        } else {
            alert("Пожалуйста, введите название комнаты!");
        }
    };

    // Подключение к комнате
    const joinRoom = async (room: string) => {
        if (!currentUserId) {
            alert("Пожалуйста, сначала установите имя пользователя.");
            return;
        }

        const roomDoc = doc(firestore, 'rooms', room);
        const roomSnapshot = await getDoc(roomDoc);

        if (roomSnapshot.exists()) {
            const users = roomSnapshot.data()?.users || [];

            if (users.includes(currentUserId)) {
                alert("Вы уже находитесь в этой комнате!");
                return; 
            }

            users.push(currentUserId);
            await updateDoc(roomDoc, { users });
            setCurrentUsers(users);
            setRoomName(room); 
        } else {
            alert("Комната с таким названием не найдена!");
        }
    };

    // Выход из комнаты
    const leaveRoom = async () => {
        if (currentUserId && roomName) {
            const roomDoc = doc(firestore, 'rooms', roomName);
            const roomSnapshot = await getDoc(roomDoc);

            if (roomSnapshot.exists()) {
                const users = roomSnapshot.data()?.users || [];
                const updatedUsers = users.filter(user => user !== currentUserId);
                await updateDoc(roomDoc, { users: updatedUsers });
                setCurrentUsers(updatedUsers);
                setRoomName(''); 
                setCreatorId(''); 
            }
        }
    };

    // Удаление комнаты
    const deleteRoom = async (room: string) => {
        const roomDoc = doc(firestore, 'rooms', room);
        const roomSnapshot = await getDoc(roomDoc);

        if (roomSnapshot.exists()) {
            const creator = roomSnapshot.data()?.creator;

            if (creator === currentUserId) {
                await deleteDoc(roomDoc);
                alert(`Комната '${room}' была удалена.`);
            } else {
                alert("Только создатель комнаты может её удалить.");
            }
        } else {
            alert("Комната не найдена.");
        }
    };

    // Загрузка списка комнат
    useEffect(() => {
        const roomsCollection = collection(firestore, 'rooms');

        const unsubscribe = onSnapshot(roomsCollection, (snapshot) => {
            const roomsList = snapshot.docs.map(doc => doc.data().id);
            setRooms(roomsList);
        });

        return () => unsubscribe();
    }, []);

    // Обновление информации о комнате
    const handleRoomUpdate = (roomName: string) => {
        const roomDoc = doc(firestore, 'rooms', roomName);
        const unsubscribe = onSnapshot(roomDoc, (snapshot) => {
            const users = snapshot.data()?.users || [];
            setCurrentUsers(users);
            setCreatorId(snapshot.data()?.creator || ''); 
        });

        return () => unsubscribe();
    };

    useEffect(() => {
        if (roomName) {
            handleRoomUpdate(roomName);
        }
    }, [roomName]);

    // Выход из комнаты при закрытии окна
    useEffect(() => {
        const handleBeforeUnload = () => {
            leaveRoom();
            return undefined;
        };

        window.addEventListener('beforeunload', handleBeforeUnload);
        return () => {
            window.removeEventListener('beforeunload', handleBeforeUnload);
        };
    }, [roomName, currentUserId]);

    // Установка имени пользователя
    const handleUsernameSubmit = () => {
        if (!username) {
            alert("Имя пользователя не может быть пустым.");
            return;
        }

        setCurrentUserId(username); 
        Cookie.set('username', username); 
        setShowUsernamePrompt(false); 
    };

    return (
        <div className="flex h-screen">
            {/* Кнопка возврата на /dashboard */}
            <motion.button 
                onClick={() => router.push('/dashboard')}
                className="absolute top-4 left-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition duration-200"
            >
                Вернуться
            </motion.button>

            {/* Состояние ввода имени пользователя */}
            {showUsernamePrompt ? (
                <div className="flex-1 flex justify-center items-center bg-gray-900 text-white p-4">
                    <div className="w-full max-w-sm">
                        <h1 className="text-2xl font-bold mb-6">Введите ваше имя пользователя</h1>
                        <input
                            type="text"
                            placeholder="Имя пользователя"
                            value={username}
                            onChange={(e) => setUsername(e.target.value)}
                            className="px-4 py-2 bg-gray-800 text-white rounded w-full"
                        />
                        <motion.button
                            onClick={handleUsernameSubmit}
                            className="mt-4 px-4 py-2 bg-green-600 hover:bg-green-700 transition duration-300 rounded w-full"
                        >
                            Подтвердить
                        </motion.button>
                    </div>
                </div>
            ) : (
                <>
                    {/* Боковая панель с комнатами */}
                    <div className="w-1/4 bg-gray-800 p-4 relative">
                    <motion.button 
                        onClick={() => router.push('/dashboard')}
                        className=" bg-blue-500 text-white rounded hover:bg-blue-600 transition duration-200"
                    >
                        Вернуться
                    </motion.button>
                        <h2 className="text-lg font-bold mb-4 text-white">Доступные комнаты</h2>
                        <ul className="space-y-2">
                            {rooms.map((room) => (
                                <li key={room} className="flex items-center justify-between">
                                    <motion.button 
                                        onClick={() => joinRoom(room)} 
                                        className="flex items-center w-full text-left p-2 rounded-full transition duration-300 hover:bg-gray-700"
                                    >
                                        <div 
                                            className="rounded-full w-8 h-8 flex items-center justify-center bg-blue-600 text-white font-bold mr-2"
                                        >
                                            {room.charAt(0).toUpperCase()}
                                        </div>
                                        <span className="text-white">{room}</span>
                                    </motion.button>
                                    {currentUserId === creatorId && (
                                        <motion.button
                                            onClick={() => deleteRoom(room)}
                                            className="text-red-500 ml-2"
                                        >
                                            🗑️ 
                                        </motion.button>
                                    )}
                                </li>
                            ))}
                        </ul>

                        {/* Кнопка добавления комнаты */}
                        <motion.button
                            onClick={() => setShowCreateRoom(!showCreateRoom)}
                            className="absolute bottom-4 left-1/2 transform -translate-x-1/2 flex items-center justify-center w-10 h-10 bg-green-500 hover:bg-green-600 rounded-full transition duration-300"
                        >
                            <span className="text-white text-2xl">+</span>
                        </motion.button>

                        {/* Форма создания комнаты */}
                        {showCreateRoom && (
                            <div className="mt-4">
                                <input
                                    type="text"
                                    placeholder="Введите название комнаты"
                                    value={roomName}
                                    onChange={(e) => setRoomName(e.target.value)}
                                    className="px-4 py-2 bg-gray-800 text-white rounded w-full"
                                />
                                <motion.button
                                    onClick={createRoom}
                                    className="mt-2 px-4 py-2 w-full bg-blue-600 hover:bg-blue-700 transition duration-300 rounded"
                                >
                                    Создать комнату
                                </motion.button>
                            </div>
                        )}
                    </div>

                    {/* Основное содержимое */}
                    <div className="flex-1 flex flex-col items-center justify-center bg-gray-900 text-white p-4">
                        {currentUserId ? (
                            <>
                                <h1 className="text-2xl font-bold mb-6">Вы в комнате: {roomName}</h1>
                                <div className="grid grid-cols-3 gap-4 mt-6">
                                    {currentUsers.map((user, index) => (
                                        <div
                                            key={user}
                                            className="flex flex-col items-center justify-center bg-gray-800 rounded-lg p-4"
                                        >
                                            <div className="rounded-full w-24 h-24 flex items-center justify-center mb-2" style={{ backgroundColor: getColor(index) }}>
                                                <span className="text-white text-xl">{user.charAt(0)}</span>
                                            </div>
                                            <p className="text-white">{user}</p>
                                        </div>
                                    ))}
                                </div>
                                <motion.button
                                    onClick={leaveRoom}
                                    className="mt-4 px-4 py-2 bg-red-600 hover:bg-red-700 transition duration-300 rounded"
                                >
                                    Уйти из комнаты
                                </motion.button>
                            </>
                        ) : (
                            <h1 className="text-2xl font-bold mb-6">Вы не находитесь в комнате</h1>
                        )}
                    </div>
                </>
            )}
        </div>
    );
};

export default Calls;
