import styles from './bells_block.module.css';

import {useState} from 'react';

export default function BellsBlock() {
    const [isOpen, setIsOpen] = useState(false);

    return (
        <div className={styles.bells_block}>
            <div className={styles.header}>
                <div>
                    <p className={styles.all_bells_h}>7</p>
                    <p className={styles.all_bells_p}>Звонков</p>
                </div>
                <button className={styles.view_all} onClick={() => setIsOpen(true)}>
                    Смотреть все
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <path d="M6 3.3335L10 8.00016L6 12.6668" stroke="#969696" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                </button>
            </div>

            <div className={styles.bells}>
                <div className={styles.bell}>
                    <p className={styles.bell_min_p}>0.</p>
                    <p>7:30 - 8:20</p>
                </div>

                <div className={styles.bell}>
                    <p className={styles.bell_min_p}>1.</p>
                    <p>8:30 - 9:50</p>
                </div>

                <div className={styles.bell}>
                    <p className={styles.bell_min_p}>2.</p>
                    <p>10:10 - 11:30</p>
                </div>

                <div className={styles.bell}>
                    <p className={styles.bell_min_p}>3.</p>
                    <p>12:00 - 13:20</p>
                </div>

                <div className={styles.bell}>
                    <p className={styles.bell_min_p}>4.</p>
                    <p>13:40 - 15:00</p>
                </div>

                <div className={styles.bell}>
                    <p className={styles.bell_min_p}>5.</p>
                    <p>15:20 - 16:40</p>
                </div>

                <div className={styles.bell}>
                    <p className={styles.bell_min_p}>6.</p>
                    <p>16:50 - 18:10</p>
                </div>
                
            </div>

        </div>
    );
}