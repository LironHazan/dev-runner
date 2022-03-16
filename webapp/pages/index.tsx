import type { NextPage } from 'next'
import styles from '../styles/Home.module.css'

const Home: NextPage = () => {
  return (
    <div className={styles.container}>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Dev-Runner UI
        </h1>

        <p className={styles.description}>
          Trigger development tasks &rarr;
          <code className={styles.code}>npm beta</code>
        </p>

        <div className={styles.grid}>
          <div className={styles.card}>
            <h2>Set Config Data</h2>
          </div>
        </div>
      </main>

      <footer className={styles.footer}>

      </footer>
    </div>
  )
}

export default Home
