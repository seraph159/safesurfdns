import type { NextPage } from 'next';
import DnsSwitcher from '../components/DnsSwitcher';
import SafeSurfFullLogo from '../icon/SafeSurfFullLogo';
import styles from './page.module.css';


const Home: NextPage = () => {
  return (
    <div className={styles.main_container}>
      <SafeSurfFullLogo height="100" width="500" />
      <DnsSwitcher />
    </div>
  );
};

export default Home;
