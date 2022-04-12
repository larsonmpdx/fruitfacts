// a regular button that's handled with onClick() or something externally

import styles from '../styles/Button.module.css';

export default function Button({ enabled, label, ...rest }) {
  return (
    <>
      {enabled ? (
        <button {...rest} className={`${styles.btn} ${styles['btn-blue']}`}>
          {label}
        </button>
      ) : (
        <button {...rest} disabled className={`${styles.btn} ${styles['btn-disabled']} disabled`}>
          {label}
        </button>
      )}
    </>
  );
}
