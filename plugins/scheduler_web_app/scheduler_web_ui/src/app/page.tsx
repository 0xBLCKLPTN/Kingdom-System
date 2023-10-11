import Image from 'next/image'
import styles from './page.module.css'
import UserCredentialsForm from './Components/InputField'

export default function Home() {
  return (
    <main className={styles.main}>
      <div className={styles.title}>
        <h1>Kingdom System</h1>
        <UserCredentialsForm />
        
      </div>
    </main>
  )
}
