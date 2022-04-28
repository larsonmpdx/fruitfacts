import Link from 'next/link';
import Search from './navbarSearch';
import Login from './navbarLogin';
import Tooltip from '@mui/material/Tooltip';
import React from 'react';

let links = [
  { name: 'locations', href: '/dirs/' },
  { name: 'plants', href: '/plants' },
  { name: 'US patents', href: '/patents/0' }
];

export default function Home({ user, setUser, contributingLinks }) {
  const [open, setOpen] = React.useState(false);
  const handleEditClick = () => {
    setOpen(!open);
  };

  return (
    <nav className="flex flex-wrap items-center justify-between bg-teal-500 p-6">
      <div className="mr-6 flex flex-shrink-0 items-center text-white">
        <Link href="/">
          <a className="text-xl font-semibold tracking-tight">fruitfacts</a>
        </Link>
      </div>
      <div className="mr-6 flex flex-shrink-0 items-center text-white">
        <Search />
      </div>
      <div className="block w-full flex-grow lg:flex lg:w-auto lg:items-center">
        <div className="text-sm lg:flex-grow">
          {links.map((link) => (
            <Link key={link.name} href={link.href}>
              <a className="mt-4 mr-4 block text-teal-200 hover:text-white lg:mt-0 lg:inline-block">
                {link.name}
              </a>
            </Link>
          ))}
        </div>
        <div className="mr-6">
          <Tooltip
            open={open}
            title={
              <React.Fragment>
                <a href={`${process.env.NEXT_PUBLIC_GITHUB_HOMEPAGE}`}>fruitfacts on github</a>
                <ul className="list-disc">
                  {contributingLinks && (
                    <>
                      {contributingLinks.map((link, index) => (
                        <li key={index}>
                          <a href={`${process.env.NEXT_PUBLIC_GITHUB_BASE}${link.link}`}>
                            {link.description}
                          </a>
                        </li>
                      ))}
                    </>
                  )}
                </ul>
              </React.Fragment>
            }
          >
            <button onClick={handleEditClick}>edit this page</button>
          </Tooltip>
        </div>
        <div>
          <Login user={user} setUser={setUser} />
        </div>
      </div>
    </nav>
  );
}
