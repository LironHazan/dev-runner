import type { NextPage } from 'next'
import styles from '../styles/Main.module.css'
import VerticalLinearStepper from "./runner-stepper/stepper";

const Home: NextPage = () => {
  return (
    <div className={styles.container}>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Dev-Runner UI
        </h1>

        <p className={styles.description}>
          <code className={styles.code}>npm beta</code>
        </p>

        <VerticalLinearStepper/>
      </main>

      <footer className={styles.footer}>

      </footer>
    </div>
  )
}

export default Home
