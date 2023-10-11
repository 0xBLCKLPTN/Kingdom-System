import styles from './credentials_inputs.module.css';

export default function CredentialsInputs() {
    return (
        <div className={styles.main}>
            <input className={styles.input} type="email" placeholder='E-Mail'/>
            <input className={styles.input} type="password" placeholder='Password'/>
        </div>
        
    );
}