import Link from 'next/link';
import styles from '../styles/Button.module.css';

export default function Button({ href, enabled, label }) {
  return (
    <>
      {enabled ? (
        <Link href={`${href}`}>
          <a>
            <button className={`${styles.btn} ${styles['btn-blue']}`}>{label}</button>
          </a>
        </Link>
      ) : (
        <button className={`${styles.btn} ${styles['btn-disabled']} disabled`}>{label}</button>
      )}
    </>
  );
}
