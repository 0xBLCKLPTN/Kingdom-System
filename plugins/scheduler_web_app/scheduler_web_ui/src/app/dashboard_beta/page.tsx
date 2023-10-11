import styles from './page.module.css';


export default function DashboardBeta() {
    return (
        <main className={styles.main}>
            <div className={styles.content}>
              <div className={styles.header}>
                <div>
                  <p className={styles.page_title}>Расписание</p>
                </div>
                <p className={styles.about_site}>О сайте</p>
              </div>


              <div className={styles.content_data}>
                <div className={styles.main_schedule_block}>
                    <div className={styles.block_header}>
                        <input className={styles.date_select} type="text"/>
                        <select name="course_number" id="course_number" className={styles.select_course}>
                            <option value="1">1 курс</option>
                            <option value="2">2 курс</option>
                            <option value="3">3 курс</option>
                            <option value="4">4 курс</option>
                        </select>

                        <select name="group" id="group" className={styles.select_course}>
                            <option value="1">1ПР1</option>
                            <option value="2">1КЭ1</option>
                            <option value="3">1ДО1</option>
                            <option value="4">1ТА2</option>
                        </select>
                        <input className={styles.find_by_group} type="text"/>
                        <button className={styles.primary_button}>Посмотреть расписание</button>
                    </div>

                    <div className={styles.schedule}>
                        <p>2. Психология</p>
                        <p>3. Программирование</p>
                    </div>

                    
                    
                </div>
              </div>
            </div>
        </main>
    );
}