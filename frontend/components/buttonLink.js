// a button that actually works using a nextjs Link and an html <a> tag
// use the other button component for regular onClick() buttons styled the same way

import Link from 'next/link';
import styles from '../styles/Button.module.css';

export default function Button({ href, enabled, label, ...rest }) {
  return (
    <>
      {enabled ? (
        <Link href={`${href}`}>
          <a>
            <button {...rest} className={`${styles.btn} ${styles['btn-blue']}`}>
              {label}
            </button>
          </a>
        </Link>
      ) : (
        <button {...rest} className={`${styles.btn} ${styles['btn-disabled']} disabled`}>
          {label}
        </button>
      )}
    </>
  );
}
