"use client"; // This is a client component

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import styles from './DnsSwitcher.module.css';

interface ModalProps {
  show: boolean;
  message: string;
  status: 'ON' | 'OFF';
}

const dnsProviders = [
  { name: 'CleanBrowsing DNS', primary: '185.228.168.168', secondary: '185.228.169.168'},
  { name: 'OpenDNS FamilyShield', primary: '208.67.222.222', secondary: '208.67.220.220'},
  { name: 'AdGuard DNS', primary: '176.103.130.132', secondary: '176.103.130.134'},
  { name: 'Neustar DNS', primary: '156.154.70.3', secondary: '156.154.71.3'},
  { name: 'DNS for Family', primary: '94.130.180.225', secondary: '78.47.64.161'},

  // Add other DNS providers here
];

const DnsSwitcher = () => {
  const [selectedDns, setSelectedDns] = useState(dnsProviders[0]);
  const [primaryDNS, setPrimaryDNS] = useState(dnsProviders[0].primary);
  const [secondaryDNS, setSecondaryDNS] = useState(dnsProviders[0].secondary);
  const [status, setStatus] = useState('OFF');
  const [showModal, setShowModal] = useState(false);
  const [modalMessage, setModalMessage] = useState('');
  const [modalColor, setModalColor] = useState('g');


  const Modal: React.FC<ModalProps>  = ({ show, message, status }) => {
    if (!show) return null;
  
    return (
      <div className={status === 'ON' ? styles.greenModal : styles.redModal}>
        {message}
      </div>
    );
  };
  

  const applyDns = async () => {
    // Call the Tauri function to update DNS settings
    try {
        await invoke('set_safe_dns', {
          primarydns: selectedDns.primary,
          secondarydns: selectedDns.secondary,
        });
        alert('DNS settings updated successfully');
        setModalMessage('DNS settings updated successfully');
        setShowModal(true);
        setModalColor('green');
        setStatus('ON');
      } catch (error) {
        console.error('Failed to update DNS settings:', error);
        alert('Failed to update DNS settings');
        setModalMessage('Failed to update DNS settings');
        setShowModal(true);
        setModalColor('red');
        setStatus('OFF');
      }
    };

  const resetDns = async () => {
      // Call the Tauri function to reset DNS settings
      try {
          await invoke('set_default_dns', {
            primarydns: "none",
            secondarydns: "none",
          });
          alert('DNS settings reseted successfully');
          setModalMessage('DNS settings reset successfully');
          setShowModal(true);
          setModalColor('green');
          setStatus('OFF');
        } catch (error) {
          console.error('Failed to reset DNS settings:', error);
          alert('Failed to reset DNS settings');
          setModalMessage('Failed to reset DNS settings');
          setModalColor('red');
          setShowModal(true);
          if (status == 'ON')
            setStatus('ON');
          else 
            setStatus('OFF');
        }
  };

  const handleSelectChange = (e: { target: { value: string; }; }) => {
    const selectedProvider = dnsProviders.find(
      (provider) => provider.name === e.target.value
    ) || {
      name: '',
      primary: '',
      secondary: '',
    };
    setSelectedDns(selectedProvider);
    setPrimaryDNS(selectedProvider.primary);
    setSecondaryDNS(selectedProvider.secondary);
    setShowModal(false); // Hide the modal when the select option changes
  };
  
    
  return (
    <div className={styles.fullWindow}>
      <div className={styles.container}>
      <div className={styles.selectionContainer}>
      <div className={styles.flexContainer}>
      <label htmlFor="dnsSelect" className={styles.dnsSelect}>Choose DNS:</label>
        <select
          id="dnsSelect"
          value={selectedDns.name}
          className={styles.select}
          onChange={handleSelectChange}
        >
          {dnsProviders.map((provider) => (
            <option className={styles.option} key={provider.name} value={provider.name}>
              {provider.name}
            </option>
          ))}
        </select>
        </div>
        <div className={styles.dnsInfo}>
          <p>Primary DNS: {primaryDNS}</p>
          <p>Secondary DNS: {secondaryDNS}</p>
        </div>
        {/* Display primary and secondary DNS */}
      </div>
      <div className={styles.buttonContainer}>
        <button className={styles.button} onClick={applyDns}>Enable DNS</button>
        <button className={styles.button} onClick={resetDns}>Reset DNS</button>
      </div>
      <div className={`${styles.statusContainer} ${status === 'ON' ? styles.statusOn : status === 'OFF' ? styles.statusOff : ''}`}>
          <div className={styles.statusText}>Status:</div>
          <div className={styles.statusGlow}>{status}</div>
        </div>
        </div>
        <Modal show={showModal} message={modalMessage} status={modalColor === "green" ? "ON" : "OFF"}/>
    </div>
  );
};

export default DnsSwitcher;
