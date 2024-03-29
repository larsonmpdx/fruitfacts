// see react-leaflet examples here https://tomik23.github.io/react-leaflet-examples/#/simple-map

import React from 'react';
import Link from 'next/link';
import { MapContainer, Marker, Popup, TileLayer, useMapEvents } from 'react-leaflet';
import L from 'leaflet';
import useSupercluster from 'use-supercluster';
import 'leaflet/dist/leaflet.css';
import Button from '../button';
import { name_to_path } from '../util';
import { locations_to_geoJSON } from './util';

function GetLocations({ map, setClick, setExtents, setZoom, setCenter }) {
  useMapEvents({
    click(e) {
      setClick(e.latlng);
    },
    locationfound() {
      console.log("got user's location"); // todo
    }
  });

  React.useEffect(() => {
    if (!map) return;

    setExtents(map.getBounds()); // initial
    setZoom(map.getZoom());
    setCenter(map.getCenter());

    map.on('moveend zoomend', () => {
      setExtents(map.getBounds());
      setZoom(map.getZoom());
      setCenter(map.getCenter());
    });
  }, [map]);

  return <></>;
}

function GetLeaves({ cluster, supercluster }) {
  const [leaves, setLeaves] = React.useState([]);
  const [pageNum, setPageNum] = React.useState(0);
  const PER_PAGE = 5;

  React.useEffect(() => {
    console.log(`pagenum ${pageNum}`);
    const leaves_ = supercluster.getLeaves(cluster.id, PER_PAGE, pageNum * PER_PAGE);
    console.log(leaves_);
    setLeaves(leaves_);
  }, [cluster, pageNum]);

  console.log(`pagenum ${pageNum}, ${(cluster.properties.point_count, null, 2)} total points`);
  return (
    <div>
      <ul className="list-disc">
        {leaves.map((leaf) => (
          <li
            className="leading-none"
            key={`point-${leaf.properties.collection_path}${leaf.properties.collection_title}/${leaf.properties.location_number}`}
          >
            <Link
              href={`/collections/${leaf.properties.collection_path}${name_to_path(
                leaf.properties.collection_filename
              )}?loc=${leaf.properties.location_number}`}
              className="text-xs font-semibold leading-none tracking-tight"
            >
              {`${leaf.properties.collection_title}`}
            </Link>
          </li>
        ))}
      </ul>
      {cluster.properties.point_count > PER_PAGE && (
        <>
          <Button
            onClick={() => {
              setPageNum(pageNum - 1);
            }}
            enabled={pageNum > 0}
            label="previous"
          />
          <Button
            onClick={() => {
              setPageNum(pageNum + 1);
            }}
            enabled={pageNum < Math.ceil(cluster.properties.point_count / PER_PAGE) - 1}
            label="next"
          />
        </>
      )}
    </div>
  );
}

const icons = {};
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
    });
  }
  return icons[count];
};

const getFruitIcon = (type, size) => {
  if (!icons[type]) {
    icons[type] = new L.DivIcon({
      className: 'fruiticon',
      iconSize: [size, size],
      iconAnchor: [12, 24],
      popupAnchor: [7, -16],
      html: `<img src="/fruit_icons/${type}.svg" style="width: ${size}px; height: ${size}px;" />`
    });
  }
  return icons[type];
};

export default function Home({
  locations,
  initialLocation,
  setClick,
  setExtentsForFetch,
  setZoomForQuery,
  setCenterForQuery
}) {
  const [extents, setExtents] = React.useState(null);
  const [clusterBounds, setClusterBounds] = React.useState(null);
  const [zoom, setZoom] = React.useState(3);
  const [center, setCenter] = React.useState({});

  React.useEffect(() => {
    if (!extents) {
      return;
    }

    let bounds = [
      extents._southWest.lng,
      extents._southWest.lat,
      extents._northEast.lng,
      extents._northEast.lat
    ];
    setExtentsForFetch(bounds);

    // convert leaflet extents to the bounds format supercluster wants
    setClusterBounds(bounds);
  }, [extents]);

  React.useEffect(() => {
    if (!zoom) {
      return;
    }
    setZoomForQuery(zoom);
  }, [zoom]);

  React.useEffect(() => {
    if (!center) {
      return;
    }
    setCenterForQuery(center);
  }, [center]);

  const { clusters, supercluster } = useSupercluster({
    points: locations_to_geoJSON(locations),
    bounds: clusterBounds,
    zoom,
    options: { radius: 75, maxZoom: 20 }
  });

  // console.log('clusters: ' + JSON.stringify(clusters, null, 2))

  const [map, setMap] = React.useState(null);

  let initialLat = 40.5;
  let initialLon = -100;
  let initialZoom = 3;
  if (initialLocation?.lat && initialLocation?.lon && initialLocation?.zoom) {
    initialLat = initialLocation.lat;
    initialLon = initialLocation.lon;
    initialZoom = initialLocation.zoom;
  }

  return (
    <MapContainer
      scrollWheelZoom={true}
      style={{ height: '80vh', width: '100%' }}
      center={[initialLat, initialLon]}
      zoom={initialZoom}
      ref={setMap}
    >
      <TileLayer
        attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
      <GetLocations
        map={map}
        setClick={setClick}
        setExtents={setExtents}
        setZoom={setZoom}
        setCenter={setCenter}
      />

      {clusters.map((cluster) => {
        const [longitude, latitude] = cluster.geometry.coordinates;
        const { cluster: isCluster, point_count: pointCount } = cluster.properties;

        if (isCluster) {
          return (
            <Marker
              key={`cluster-${cluster.id}`}
              position={[latitude, longitude]}
              icon={getClusterIcon(pointCount, 40)}
              eventHandlers={{
                click() {
                  // map.panTo(e.target.getLatLng());
                }
              }}
            >
              <Popup autoPan={false} minWidth={250}>
                <GetLeaves cluster={cluster} supercluster={supercluster} />
              </Popup>
            </Marker>
          );
        }

        return (
          <Marker
            key={`point-${cluster.properties.collection_path}${cluster.properties.collection_title}/${cluster.properties.location_number}`}
            position={[latitude, longitude]}
            icon={getFruitIcon('Apple', 20)}
            eventHandlers={{
              click() {
                // map.panTo(e.target.getLatLng());
              }
            }}
          >
            <Popup autoPan={false} minWidth={250}>
              <Link
                href={`/collections/${cluster.properties.collection_path}${name_to_path(
                  cluster.properties.collection_filename
                )}?loc=${cluster.properties.location_number}`}
                className="space-y-0 text-xs font-semibold leading-none tracking-tight"
              >
                {`${cluster.properties.location_name}: ${cluster.properties.collection_title}`}
              </Link>
            </Popup>
          </Marker>
        );
      })}
    </MapContainer>
  );
}
