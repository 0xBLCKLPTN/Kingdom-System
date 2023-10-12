import styles from './tasks_block.module.css';

import Modal from 'react-modal';
import { useEffect, useState } from 'react';
import fetch from 'node-fetch';
import axios from 'axios';


export default function TasksBlock() {
    const [isOpen, setIsOpen] = useState(false);
    const [lessons, setLessons] = useState([]);

    useEffect(() => {
        const fetchData = async () => {
            const data = await fetch(`http://127.0.0.1:4000/api/lessons`, { mode: 'cors'})
                .then((response) => response.json());
            setLessons(data['data'])
            console.log(data['data']);
        }

        fetchData().catch(console.error);
    }, []);

    return (
        <div className={styles.tasks_block}>
            <div className={styles.tasks_data}>
                <p className={styles.tasks_block_h}>{lessons && lessons.length}</p>
                <p className={styles.tasks_block_p}>Предметов</p>
            </div>
            <button className={styles.view_all} onClick={() => setIsOpen(true)}>
                Смотреть все
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none">
                    <path d="M6 3.3335L10 8.00016L6 12.6668" stroke="#969696" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </button>

            <Modal isOpen={isOpen} onRequestClose={() => setIsOpen(false)} className={styles.tasks_modal}>
                <div className={styles.inModalContent}>
                    <div className={styles.modal_header}>
                        <p className={styles.tasks_block_h}>{lessons && lessons.length}</p>
                        <button onClick={() => setIsOpen(false)} className={styles.closeModalButton}>
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none">
                                <circle cx="12" cy="12" r="10" stroke="#969696" stroke-width="1.5"/>
                                <path d="M14.5 9.50002L9.5 14.5M9.49998 9.5L14.5 14.5" stroke="#969696" stroke-width="1.5" stroke-linecap="round"/>
                            </svg>
                        </button>
                    </div>
                    <p className={styles.tasks_block_p}>Предметов</p>
                    <div className={styles.modal_tasks}>
                    {lessons && lessons.map(obj => <div className={styles.modal_task}><p>{obj['full_name']}</p></div>)}
                        
                        

                    </div>
                </div>

            </Modal>
        </div>
    );
}