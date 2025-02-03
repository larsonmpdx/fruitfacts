// a button that actually works using a nextjs Link and an html <a> tag
// use the other button component for regular onClick() buttons styled the same way

import Link from 'next/link';
import styles from '../styles/Button.module.css';

export default function Button({ href, enabled, label, color, ...rest }) {
  if (!color) {
    color = 'blue';
  }
  return (
    <>
      {enabled ? (
        <Link href={`${href}`}>
          <button {...rest} className={`${styles.btn} ${styles['btn-' + color]}`}>
            {label}
          </button>
        </Link>
      ) : (
        <button
          {...rest}
          className={`${styles.btn} ${styles['btn-' + color + '-disabled']} disabled`}
        >
          {label}
        </button>
      )}
    </>
  );
}
