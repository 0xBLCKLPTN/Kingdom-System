import styles from './groups_block.module.css';

import Modal from 'react-modal';
import { useState, useEffect} from 'react';
import fetch from 'node-fetch';
import axios from 'axios';

export default function GroupsBlock() {
    const [isOpen, setIsOpen] = useState(false);
    const [groups, setGroups] = useState([]);
    const [selectedGroup, setSelectedGroup] = useState('');

    useEffect(() => {
        const fetchData = async () => {
            const data = await fetch(`http://127.0.0.1:4000/api/groups`, { mode: 'cors'})
                .then((response) => response.json());
            setGroups(data['data'])
            console.log(data['data']);
        }

        fetchData().catch(console.error);
    }, []);

    return (
        <div className={styles.groups_view}>
            <div className={styles.header}>
                <div>
                    <p className={styles.all_groups_h}>{groups && groups.length}</p>
                    <p className={styles.all_groups_p}>Групп всего</p>
                </div>
                <button className={styles.view_all} onClick={() => setIsOpen(true)}>
                    Смотреть все
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <path d="M6 3.3335L10 8.00016L6 12.6668" stroke="#969696" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                </button>
            </div>

            <div className={styles.search}>
                <select name="group_name" id="group_id" className={styles.selector}>
                    <option value="1">1 курс</option>
                    <option value="2">2 курс</option>
                    <option value="3">3 курс</option>
                    <option value="4">4 курс</option>
                </select>
                <select name="group_name" id="group_id" className={styles.selector}>
                    <option value="1">1 курс</option>
                    <option value="2">2 курс</option>
                    <option value="3">3 курс</option>
                    <option value="4">4 курс</option>
                </select>
                <button className={styles.search_button}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <g clip-path="url(#clip0_493_20514)">
                            <circle cx="7.66659" cy="7.66683" r="6.33333" stroke="#1C274C"/>
                            <path d="M12.3333 12.3335L14.6666 14.6668" stroke="#1C274C" stroke-linecap="round"/>
                        </g>
                        <defs>
                            <clipPath id="clip0_493_20514">
                                <rect width="16" height="16" fill="white"/>
                            </clipPath>
                        </defs>
                    </svg>
                    <p>Поиск</p>
                </button>

            </div>
            <div className={styles.lessons}>
                <div className={styles.lesson}>
                    <p className={styles.number}>1.</p>
                    <p className={styles.name}>Информационная безопас</p>
                    <p className={styles.office_number}>(409)</p>
                </div>
                <div className={styles.lesson}>
                    <p className={styles.number}>2.</p>
                    <p className={styles.name}>Психология</p>
                    <p className={styles.office_number}>(311)</p>
                </div>
                <div className={styles.lesson}>
                    <p className={styles.number}>3.</p>
                    <p className={styles.name}>Шелковникова</p>
                    <p className={styles.office_number}>(410)</p>
                </div>
            </div>

            <Modal isOpen={isOpen} onRequestClose={() => setIsOpen(false)} className={styles.groups_modal}>
                <div className={styles.inModalContent}>
                    <div className={styles.modal_header}>
                        <p className={styles.all_groups_h}>{groups && groups.length}</p>
                        <button onClick={() => setIsOpen(false)} className={styles.closeModalButton}>
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none">
                                <circle cx="12" cy="12" r="10" stroke="#969696" stroke-width="1.5"/>
                                <path d="M14.5 9.50002L9.5 14.5M9.49998 9.5L14.5 14.5" stroke="#969696" stroke-width="1.5" stroke-linecap="round"/>
                            </svg>
                        </button>
                    </div>
                    <p className={styles.all_groups_p}>Групп всего</p>
                    <div className={styles.modal_datas}>
                        <p className={styles.course_number}>1 Курс</p>
                        <div className={styles.modal_groups}>
                            {groups && groups.map(obj => {
                                if (obj['full_name'].startsWith('1')) {
                                    return (<div className={styles.modal_group}>
                                        <button onClick={() => (setSelectedGroup(obj['full_name']), setIsOpen(false))}>{obj['full_name']}</button>
                                    </div>);
                                }
                            })}
                        </div>

                        <p className={styles.course_number}>2 Курс</p>
                        <div className={styles.modal_groups}>
                            {groups && groups.map(obj => {
                                if (obj['full_name'].startsWith('2')) {
                                    return (<div className={styles.modal_group}>
                                        <button onClick={() => (setSelectedGroup(obj['full_name']), setIsOpen(false))}>{obj['full_name']}</button>
                                    </div>);
                                }
                            })}
                        </div>

                        <p className={styles.course_number}>3 Курс</p>
                        <div className={styles.modal_groups}>
                            {groups && groups.map(obj => {
                                if (obj['full_name'].startsWith('3')) {
                                    return (<div className={styles.modal_group}>
                                        <button onClick={() => (setSelectedGroup(obj['full_name']), setIsOpen(false))}>{obj['full_name']}</button>
                                    </div>);
                                }
                            })}
                        </div>

                        <p className={styles.course_number}>4 Курс</p>
                        <div className={styles.modal_groups}>
                            {groups && groups.map(obj => {
                                if (obj['full_name'].startsWith('4')) {
                                    return (<div className={styles.modal_group}>
                                        <button onClick={() => (setSelectedGroup(obj['full_name']), setIsOpen(false))}>{obj['full_name']}</button>
                                    </div>);
                                }
                            })}
                        </div>
                    </div>
                    
                </div>

            </Modal>

        </div>
    );
}