'use client';

import styles from './sign_up_button.module.css';
import { redirect } from 'next/navigation';


export default function SignUpButton() {
    return (
        <div>
            <button className={styles.main} onClick={redirect('/dashboard')}>Анонимный</button>
        </div>
        
    );
}