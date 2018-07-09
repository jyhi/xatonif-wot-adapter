-- phpMyAdmin SQL Dump
-- version 4.7.9
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: 2018-07-08 07:52:13
-- 服务器版本： 10.1.31-MariaDB
-- PHP Version: 7.2.3

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
SET AUTOCOMMIT = 0;
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `xatonif`
--

-- --------------------------------------------------------

--
-- 表的结构 `device_actions`
--

CREATE TABLE `device_actions` (
  `id` int(10) UNSIGNED NOT NULL,
  `name` text COLLATE utf8mb4_unicode_ci,
  `description` text COLLATE utf8mb4_unicode_ci
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `device_assoc_actions`
--

CREATE TABLE `device_assoc_actions` (
  `device_id` int(10) UNSIGNED NOT NULL,
  `action_id` int(10) UNSIGNED NOT NULL,
  `id` int(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `device_assoc_events`
--

CREATE TABLE `device_assoc_events` (
  `device_id` int(10) UNSIGNED NOT NULL,
  `event_id` int(10) UNSIGNED NOT NULL,
  `id` int(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `device_assoc_properties`
--

CREATE TABLE `device_assoc_properties` (
  `device_id` int(10) UNSIGNED NOT NULL,
  `prop_id` int(10) UNSIGNED NOT NULL,
  `id` int(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `device_events`
--

CREATE TABLE `device_events` (
  `id` int(10) UNSIGNED NOT NULL,
  `name` text COLLATE utf8mb4_unicode_ci,
  `description` text COLLATE utf8mb4_unicode_ci
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `device_event_logs`
--

CREATE TABLE `device_event_logs` (
  `id` int(10) UNSIGNED NOT NULL,
  `time` datetime NOT NULL,
  `dev_id` int(10) UNSIGNED NOT NULL,
  `event_id` int(10) UNSIGNED NOT NULL,
  `task_id` int(10) UNSIGNED NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `device_list`
--

CREATE TABLE `device_list` (
  `id` int(10) UNSIGNED NOT NULL,
  `name` text COLLATE utf8mb4_unicode_ci,
  `type` text COLLATE utf8mb4_unicode_ci,
  `description` text COLLATE utf8mb4_unicode_ci,
  `assoc_properties_id` int(10) UNSIGNED NOT NULL,
  `assoc_actions_id` int(10) UNSIGNED NOT NULL,
  `assoc_events_id` int(10) UNSIGNED NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `device_properties`
--

CREATE TABLE `device_properties` (
  `id` int(10) UNSIGNED NOT NULL,
  `name` text COLLATE utf8mb4_unicode_ci,
  `type` text COLLATE utf8mb4_unicode_ci,
  `description` text COLLATE utf8mb4_unicode_ci,
  `href` text COLLATE utf8mb4_unicode_ci
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `task_assoc_devs`
--

CREATE TABLE `task_assoc_devs` (
  `task_id` int(10) UNSIGNED NOT NULL,
  `dev_id` int(10) UNSIGNED NOT NULL,
  `id` int(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `task_list`
--

CREATE TABLE `task_list` (
  `id` int(10) UNSIGNED NOT NULL,
  `user_id` int(10) UNSIGNED NOT NULL,
  `start_time` datetime NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `user_action_logs`
--

CREATE TABLE `user_action_logs` (
  `id` int(10) UNSIGNED NOT NULL,
  `time` datetime NOT NULL,
  `user_id` int(10) UNSIGNED NOT NULL,
  `action_id` int(10) UNSIGNED NOT NULL,
  `dev_id` int(10) UNSIGNED NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- 表的结构 `user_list`
--

CREATE TABLE `user_list` (
  `user_id` int(10) UNSIGNED NOT NULL,
  `user_name` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `password` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Indexes for dumped tables
--

--
-- Indexes for table `device_actions`
--
ALTER TABLE `device_actions`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `device_assoc_actions`
--
ALTER TABLE `device_assoc_actions`
  ADD PRIMARY KEY (`id`),
  ADD KEY `device_id` (`device_id`),
  ADD KEY `action_id` (`action_id`);

--
-- Indexes for table `device_assoc_events`
--
ALTER TABLE `device_assoc_events`
  ADD PRIMARY KEY (`id`),
  ADD KEY `device_id` (`device_id`),
  ADD KEY `event_id` (`event_id`);

--
-- Indexes for table `device_assoc_properties`
--
ALTER TABLE `device_assoc_properties`
  ADD PRIMARY KEY (`id`),
  ADD KEY `device_id` (`device_id`),
  ADD KEY `prop_id` (`prop_id`);

--
-- Indexes for table `device_events`
--
ALTER TABLE `device_events`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `device_event_logs`
--
ALTER TABLE `device_event_logs`
  ADD PRIMARY KEY (`id`),
  ADD KEY `dev_id` (`dev_id`),
  ADD KEY `task_id` (`task_id`),
  ADD KEY `event_id` (`event_id`);

--
-- Indexes for table `device_list`
--
ALTER TABLE `device_list`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `device_properties`
--
ALTER TABLE `device_properties`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `task_assoc_devs`
--
ALTER TABLE `task_assoc_devs`
  ADD PRIMARY KEY (`id`),
  ADD KEY `task_id` (`task_id`),
  ADD KEY `dev_id` (`dev_id`);

--
-- Indexes for table `task_list`
--
ALTER TABLE `task_list`
  ADD PRIMARY KEY (`id`),
  ADD KEY `user_id` (`user_id`);

--
-- Indexes for table `user_action_logs`
--
ALTER TABLE `user_action_logs`
  ADD PRIMARY KEY (`id`),
  ADD KEY `user_id` (`user_id`),
  ADD KEY `dev_id` (`dev_id`),
  ADD KEY `action_id` (`action_id`);

--
-- Indexes for table `user_list`
--
ALTER TABLE `user_list`
  ADD PRIMARY KEY (`user_id`);

--
-- 在导出的表使用AUTO_INCREMENT
--

--
-- 使用表AUTO_INCREMENT `device_actions`
--
ALTER TABLE `device_actions`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `device_events`
--
ALTER TABLE `device_events`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `device_event_logs`
--
ALTER TABLE `device_event_logs`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `device_list`
--
ALTER TABLE `device_list`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `device_properties`
--
ALTER TABLE `device_properties`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `task_list`
--
ALTER TABLE `task_list`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `user_action_logs`
--
ALTER TABLE `user_action_logs`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `user_list`
--
ALTER TABLE `user_list`
  MODIFY `user_id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- 限制导出的表
--

--
-- 限制表 `device_assoc_actions`
--
ALTER TABLE `device_assoc_actions`
  ADD CONSTRAINT `device_assoc_actions_ibfk_1` FOREIGN KEY (`device_id`) REFERENCES `device_list` (`id`),
  ADD CONSTRAINT `device_assoc_actions_ibfk_2` FOREIGN KEY (`action_id`) REFERENCES `device_actions` (`id`);

--
-- 限制表 `device_assoc_events`
--
ALTER TABLE `device_assoc_events`
  ADD CONSTRAINT `device_assoc_events_ibfk_1` FOREIGN KEY (`device_id`) REFERENCES `device_list` (`id`),
  ADD CONSTRAINT `device_assoc_events_ibfk_2` FOREIGN KEY (`event_id`) REFERENCES `device_events` (`id`);

--
-- 限制表 `device_assoc_properties`
--
ALTER TABLE `device_assoc_properties`
  ADD CONSTRAINT `device_assoc_properties_ibfk_1` FOREIGN KEY (`device_id`) REFERENCES `device_list` (`id`),
  ADD CONSTRAINT `device_assoc_properties_ibfk_2` FOREIGN KEY (`prop_id`) REFERENCES `device_properties` (`id`);

--
-- 限制表 `device_event_logs`
--
ALTER TABLE `device_event_logs`
  ADD CONSTRAINT `device_event_logs_ibfk_1` FOREIGN KEY (`dev_id`) REFERENCES `device_list` (`id`),
  ADD CONSTRAINT `device_event_logs_ibfk_2` FOREIGN KEY (`task_id`) REFERENCES `task_list` (`id`),
  ADD CONSTRAINT `device_event_logs_ibfk_3` FOREIGN KEY (`event_id`) REFERENCES `device_events` (`id`);

--
-- 限制表 `task_assoc_devs`
--
ALTER TABLE `task_assoc_devs`
  ADD CONSTRAINT `task_assoc_devs_ibfk_1` FOREIGN KEY (`task_id`) REFERENCES `task_list` (`id`),
  ADD CONSTRAINT `task_assoc_devs_ibfk_2` FOREIGN KEY (`dev_id`) REFERENCES `device_list` (`id`);

--
-- 限制表 `task_list`
--
ALTER TABLE `task_list`
  ADD CONSTRAINT `task_list_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `user_list` (`user_id`);

--
-- 限制表 `user_action_logs`
--
ALTER TABLE `user_action_logs`
  ADD CONSTRAINT `user_action_logs_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `user_list` (`user_id`),
  ADD CONSTRAINT `user_action_logs_ibfk_2` FOREIGN KEY (`dev_id`) REFERENCES `device_list` (`id`),
  ADD CONSTRAINT `user_action_logs_ibfk_3` FOREIGN KEY (`action_id`) REFERENCES `device_actions` (`id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
