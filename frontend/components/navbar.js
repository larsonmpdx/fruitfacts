import React from 'react';
import Link from 'next/link';
import { Section, Navbar } from 'react-bulma-components';

let links = [
    { name: 'locations', href: '/dirs/' },
    { name: 'plants', href: '/plants' },
    { name: 'US patents', href: '/patents/0' }
];

export default function Home() {
    return (
        <Section>
            <Navbar>
                <Navbar.Brand>
                    <Navbar.Item renderAs="a" href="#">
                        <img
                            src="https://bulma.io/images/bulma-logo.png"
                            alt="Bulma: a modern CSS framework based on Flexbox"
                            width="112"
                            height="28"
                        />
                    </Navbar.Item>
                    <Navbar.Burger />
                </Navbar.Brand>
                <Navbar.Menu>
                    <Navbar.Container>
                        {links.map((link) => (
                            <Navbar.Item href={link.href}>{link.name}</Navbar.Item>
                        ))}
                    </Navbar.Container>
                    <Navbar.Container position="end">
                        <Navbar.Item href="#">At the end</Navbar.Item>
                    </Navbar.Container>
                </Navbar.Menu>
            </Navbar>
        </Section>
    );
}
