import Image from 'next/image'
import styles from './page.module.css'
import CredentialsInputs from '../Components/Inputs/credentials_inputs/CredentialsInputs'
import SignInButton from '../Components/Buttons/sign_in_button/SignInButton'
import SignUpButton from '../Components/Buttons/sign_up_button/SignUpButton'

export default function Auth() {
  return (
    <main className={styles.main}>
      <div>
        <h1 className={styles.title} >Kingdom-System</h1>
        <CredentialsInputs />
        <div className={styles.buttons}>
          <SignInButton/>
          <SignUpButton/>
        </div>
      </div>
    </main>
  )
}
