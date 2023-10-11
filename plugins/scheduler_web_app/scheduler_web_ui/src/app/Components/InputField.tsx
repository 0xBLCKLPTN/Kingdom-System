import styles from './inputfield.module.css';

export default function UserCredentialsForm() {
    return (
        <div>
            <div className={styles.input_data}>
                <input className={styles.input_field} type="email" placeholder='Email'/>
                <input className={styles.input_field} type="password" placeholder='Password'/>
            </div>
            <div className={styles.buttons}>
                <button className={styles.sign_in_button}>Войти</button>
                <button className={styles.sign_up_button}>Зарегистрироваться</button>
            </div>
        </div>
        
    );
}