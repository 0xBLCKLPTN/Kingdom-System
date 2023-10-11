import styles from './dashboard.module.css';

export default function Dashboard() {
    return (
        <main className={styles.main}>
            <div className={styles.content}>
              <div className={styles.header}>
                <div>
                  <p className={styles.page_title}>Панель управления</p>
                  <p className={styles.for_students}>Открыть версию для студентов</p>
                </div>
                <p className={styles.about_site}>О сайте</p>
              </div>


              <div className={styles.content_data}>
                <div className={styles.calendar}>

                </div>
              </div>
            </div>
        </main>
    )
}