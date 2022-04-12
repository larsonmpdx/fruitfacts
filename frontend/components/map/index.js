// see react-leaflet examples here https://tomik23.github.io/react-leaflet-examples/#/simple-map

import React, { useCallback } from 'react'
import Link from 'next/link'
import { MapContainer, Marker, Popup, TileLayer, useMapEvents, useMap } from 'react-leaflet'
import L from 'leaflet'
import useSupercluster from 'use-supercluster'
import 'leaflet/dist/leaflet.css'
import styles from '../../styles/Map.module.css'
import Button from '../button'

import { locations_to_geoJSON } from './util'

function GetLocations ({ map, setClick, setExtents, setZoom }) {
  useMapEvents({
    click (e) {
      setClick(e.latlng)
    },
    locationfound (e) {
      console.log("got user's location") // todo
    }
  })

  React.useEffect(() => {
    if (!map) return

    setExtents(map.getBounds()) // initial
    setZoom(map.getZoom())

    map.on('moveend zoomend', () => {
      setExtents(map.getBounds())
      setZoom(map.getZoom())
    })
  }, [map])

  return <></>
}

function GetLeaves ({ cluster, supercluster }) {
  const [leaves, setLeaves] = React.useState([])
  const [pageNum, setPageNum] = React.useState(0)
  const PER_PAGE = 5

  React.useEffect(() => {
    console.log(`pagenum ${pageNum}`)
    const leaves_ = supercluster.getLeaves(cluster.id, PER_PAGE, pageNum * PER_PAGE)
    console.log(leaves_)
    setLeaves(leaves_)
  }, [cluster, pageNum])

  console.log(`pagenum ${pageNum}, ${(cluster.properties.point_count, null, 2)} total points`)
  return (
    <div>
      <ul className='list-disc'>
        {leaves.map(leaf => (
          <li
            key={`point-${leaf.properties.collection_path}${leaf.properties.collection_title}/${leaf.properties.location_number}`}
          >
            <Link
              href={`/collections/${leaf.properties.collection_path}${encodeURIComponent(
                leaf.properties.collection_filename
              )}?loc=${leaf.properties.location_number}`}
            >
              <a className='text-2xs font-semibold tracking-tight'>{`${leaf.properties.collection_title}`}</a>
            </Link>
          </li>
        ))}
      </ul>
      <Button
        onClick={() => {
          setPageNum(pageNum - 1)
        }}
        enabled={pageNum > 0}
        label='previous'
      />
      <Button
        onClick={() => {
          setPageNum(pageNum + 1)
        }}
        enabled={cluster.properties.point_count > PER_PAGE} // todo limit
        label='next'
      />
    </div>
  )
}

const icons = {}
const getClusterIcon = (count, size) => {
  if (!icons[count]) {
    icons[count] = new L.DivIcon({
      className: 'clustericon',
      iconSize: [size, size],
      iconAnchor: [12, 24],
      popupAnchor: [7, -16],
      html: `
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
        <circle cx="50" cy="50" r="20" stroke="black" stroke-width="2" fill="none" />
        <text x="35" y="55" font-size="2em">${count}</text>
      </svg>`
    })
  }
  return icons[count]
}

const getFruitIcon = (type, size) => {
  if (!icons[type]) {
    icons[type] = new L.DivIcon({
      className: 'fruiticon',
      iconSize: [size, size],
      iconAnchor: [12, 24],
      popupAnchor: [7, -16],
      html: `<img src="/fruit_icons/${type}.svg" style="width: ${size}px; height: ${size}px;" />`
    })
  }
  return icons[type]
}

export default function Home ({ locations, setClick, setExtentsForFetch }) {
  const [extents, setExtents] = React.useState(null)
  const [clusterBounds, setClusterBounds] = React.useState(null)
  const [zoom, setZoom] = React.useState(3)

  React.useEffect(() => {
    if (!extents) {
      return
    }

    let bounds = [
      extents._southWest.lng,
      extents._southWest.lat,
      extents._northEast.lng,
      extents._northEast.lat
    ]
    setExtentsForFetch(bounds)

    // convert leaflet extents to the bounds format supercluster wants
    setClusterBounds(bounds)
  }, [extents])

  const { clusters, supercluster } = useSupercluster({
    points: locations_to_geoJSON(locations),
    bounds: clusterBounds,
    zoom,
    options: { radius: 75, maxZoom: 20 }
  })

  // console.log('clusters: ' + JSON.stringify(clusters, null, 2))

  const [map, setMap] = React.useState(null)

  return (
    <MapContainer
      zoom={3}
      scrollWheelZoom={true}
      style={{ height: 400, width: '100%' }}
      center={[40.5, -100]}
      whenCreated={setMap}
    >
      <TileLayer
        attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
        url='https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png'
      />
      <GetLocations map={map} setClick={setClick} setExtents={setExtents} setZoom={setZoom} />

      {clusters.map(cluster => {
        const [longitude, latitude] = cluster.geometry.coordinates
        const { cluster: isCluster, point_count: pointCount } = cluster.properties

        if (isCluster) {
          return (
            <Marker
              key={`cluster-${cluster.id}`}
              position={[latitude, longitude]}
              icon={getClusterIcon(pointCount, 40)}
              eventHandlers={{
                click (e) {
                  //   map.panTo(e.target.getLatLng());
                }
              }}
            >
              <Popup autoPan={false} minWidth={250}>
                <GetLeaves cluster={cluster} supercluster={supercluster} />
              </Popup>
            </Marker>
          )
        }

        return (
          <Marker
            key={`point-${cluster.properties.collection_path}${cluster.properties.collection_title}/${cluster.properties.location_number}`}
            position={[latitude, longitude]}
            icon={getFruitIcon('Apple', 20)}
            eventHandlers={{
              click (e) {
                //      map.panTo(e.target.getLatLng());
              }
            }}
          >
            <Popup autoPan={false} minWidth={250}>
              <Link
                href={`/collections/${cluster.properties.collection_path}${encodeURIComponent(
                  cluster.properties.collection_filename
                )}?loc=${cluster.properties.location_number}`}
              >
                <a className='text-xs font-semibold tracking-tight'>{`${cluster.properties.location_name}: ${cluster.properties.collection_title}`}</a>
              </Link>
            </Popup>
          </Marker>
        )
      })}
    </MapContainer>
  )
}
