'use client';

import styles from './dashboard.module.css';
import GroupsBlock from '../Components/Blocks/groups_dashboard_block/GroupsBlock';
import TeachersBlock from '../Components/Blocks/teachers_dashboard_block/TeachersBlock';
import TasksBlock from '../Components/Blocks/tasks_dashboard_block/TasksBlocks';

import Modal from 'react-modal';
import { useState } from 'react';

export default function Dashboard() {
    return (
        <main className={styles.main}>
            <div className={styles.content}>
              <div className={styles.header}>
                <div className={styles.title}>
                  <p className={styles.page_title}>Расписание для студентов</p>
                  <p className={styles.for_students}>Открыть старую версию</p>
                </div>
                <p className={styles.about_site}>О сайте</p>
              </div>


              <div className={styles.content_data}>
                <div className={styles.calendar}>

                </div>
                <GroupsBlock/>
                <div className={styles.right_column}>
                  <TeachersBlock/>
                  <TasksBlock/>

                  <div className={styles.schedules}>

                  </div>
                  
                </div>
                
              </div>
            </div>
        </main>
    )
}