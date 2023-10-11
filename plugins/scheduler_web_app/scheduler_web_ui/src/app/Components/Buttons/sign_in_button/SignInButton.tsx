import styles from './sign_in_button.module.css';

export default function SignInButton() {
    return (
        <div>
            <button className={styles.main}>Войти</button>
        </div>
        
    );
}