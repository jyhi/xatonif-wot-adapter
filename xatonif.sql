-- phpMyAdmin SQL Dump
-- version 4.8.2
-- https://www.phpmyadmin.net/
--
-- Host: localhost
-- Generation Time: 2018-07-12 21:14:22
-- 服务器版本： 10.0.34-MariaDB-0ubuntu0.16.04.1
-- PHP Version: 7.0.30-0ubuntu0.16.04.1

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

--
-- 转存表中的数据 `device_actions`
--

INSERT INTO `device_actions` (`id`, `name`, `description`) VALUES
(0, 'nil', 'This device has no action.');

-- --------------------------------------------------------

--
-- 表的结构 `device_assoc_actions`
--

CREATE TABLE `device_assoc_actions` (
  `device_id` int(10) UNSIGNED NOT NULL,
  `action_id` int(10) UNSIGNED NOT NULL,
  `id` int(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- 转存表中的数据 `device_assoc_actions`
--

INSERT INTO `device_assoc_actions` (`device_id`, `action_id`, `id`) VALUES
(0, 0, 0);

-- --------------------------------------------------------

--
-- 表的结构 `device_assoc_events`
--

CREATE TABLE `device_assoc_events` (
  `device_id` int(10) UNSIGNED NOT NULL,
  `event_id` int(10) UNSIGNED NOT NULL,
  `id` int(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- 转存表中的数据 `device_assoc_events`
--

INSERT INTO `device_assoc_events` (`device_id`, `event_id`, `id`) VALUES
(0, 0, 0);

-- --------------------------------------------------------

--
-- 表的结构 `device_assoc_properties`
--

CREATE TABLE `device_assoc_properties` (
  `device_id` int(10) UNSIGNED NOT NULL,
  `prop_id` int(10) UNSIGNED NOT NULL,
  `id` int(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- 转存表中的数据 `device_assoc_properties`
--

INSERT INTO `device_assoc_properties` (`device_id`, `prop_id`, `id`) VALUES
(0, 0, 0),
(1, 1, 1),
(2, 2, 2),
(2, 3, 3),
(3, 4, 4),
(4, 5, 5),
(1, 6, 6);

-- --------------------------------------------------------

--
-- 表的结构 `device_events`
--

CREATE TABLE `device_events` (
  `id` int(10) UNSIGNED NOT NULL,
  `name` text COLLATE utf8mb4_unicode_ci,
  `description` text COLLATE utf8mb4_unicode_ci
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- 转存表中的数据 `device_events`
--

INSERT INTO `device_events` (`id`, `name`, `description`) VALUES
(0, 'nil', 'This device has no event.');

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

--
-- 转存表中的数据 `device_list`
--

INSERT INTO `device_list` (`id`, `name`, `type`, `description`, `assoc_properties_id`, `assoc_actions_id`, `assoc_events_id`) VALUES
(0, 'dummy', 'thing', 'A dummy device', 0, 0, 0),
(1, 'hatonif-flash-light', 'onOffLight', 'The Hatonif Box Flash Light.', 1, 0, 0),
(2, 'hatonif-buzzer', 'thing', 'The Hatonif Box buzzer.', 2, 0, 0),
(3, 'hatonif-feeder', 'onOffSwitch', 'The Hatonif Box feeder.', 4, 0, 0),
(4, 'hatonif-button', 'onOffSwitch', 'The Hatonif Box button.', 5, 0, 0);

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

--
-- 转存表中的数据 `device_properties`
--

INSERT INTO `device_properties` (`id`, `name`, `type`, `description`, `href`) VALUES
(0, 'nil', NULL, 'This device has no property.', NULL),
(1, 'on', 'boolean', 'Switch the LED on / off', '/things/hatonif-flash-light/properties/on'),
(2, 'on', 'boolean', 'Switch the buzzer on / off', '/things/hatonif-buzzer/properties/on'),
(3, 'frequency', 'number', 'Specify the buzzer frequency', '/things/hatonif-buzzer/properties/frequency'),
(4, 'on', 'boolean', 'Turn the feeder box on / off', '/things/hatonif-feeder/properties/on'),
(5, 'on', 'boolean', 'Whether the button is pressed or not', '/things/hatonif-button/properties/on'),
(6, 'color', 'string', 'Specify the color of light, in hex format', '/things/hatonif-flash-light/properties/color');

-- --------------------------------------------------------

--
-- 表的结构 `task_assoc_devs`
--

CREATE TABLE `task_assoc_devs` (
  `task_id` int(10) UNSIGNED NOT NULL,
  `dev_id` int(10) UNSIGNED NOT NULL,
  `dev_status` int(10) UNSIGNED NOT NULL,
  `id` int(10) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `task_assoc_devs`
--

INSERT INTO `task_assoc_devs` (`task_id`, `dev_id`, `dev_status`, `id`) VALUES
(10, 4, 1, 9),
(10, 2, 1, 10),
(10, 4, 1, 11),
(10, 1, 1, 12);

-- --------------------------------------------------------

--
-- 表的结构 `task_edition`
--

CREATE TABLE `task_edition` (
  `id` int(15) UNSIGNED NOT NULL,
  `task_id` int(15) UNSIGNED NOT NULL,
  `if_device_id` int(15) UNSIGNED NOT NULL,
  `then_device_id` int(15) UNSIGNED NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `task_edition`
--

INSERT INTO `task_edition` (`id`, `task_id`, `if_device_id`, `then_device_id`) VALUES
(6, 10, 4, 2),
(7, 10, 4, 1);

-- --------------------------------------------------------

--
-- 表的结构 `task_list`
--

CREATE TABLE `task_list` (
  `id` int(10) UNSIGNED NOT NULL,
  `user_id` int(10) UNSIGNED NOT NULL,
  `start_time` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci,
  `task_name` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `task_sta` int(10) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- 转存表中的数据 `task_list`
--

INSERT INTO `task_list` (`id`, `user_id`, `start_time`, `description`, `task_name`, `task_sta`) VALUES
(10, 1, 'Thu Jul 12 20:50:34 2018', '就这样', '实验1', 1);

-- --------------------------------------------------------

--
-- 表的结构 `user_action_logs`
--

CREATE TABLE `user_action_logs` (
  `id` int(10) UNSIGNED NOT NULL,
  `time` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `user_id` int(10) UNSIGNED NOT NULL,
  `task_id` int(10) NOT NULL,
  `action` text COLLATE utf8mb4_unicode_ci NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- 转存表中的数据 `user_action_logs`
--

INSERT INTO `user_action_logs` (`id`, `time`, `user_id`, `task_id`, `action`) VALUES
(2, 'Wed Jul 11 21:26:58 2018', 1, 9, 'start'),
(3, 'Thu Jul 12 20:08:19 2018', 1, 9, 'stop'),
(4, 'Thu Jul 12 20:10:14 2018', 1, 9, 'start'),
(5, 'Thu Jul 12 20:10:54 2018', 1, 9, 'start'),
(6, 'Thu Jul 12 20:13:58 2018', 1, 9, 'stop'),
(7, 'Thu Jul 12 20:14:38 2018', 1, 9, 'start'),
(8, 'Thu Jul 12 20:15:11 2018', 1, 9, 'stop'),
(9, 'Thu Jul 12 20:48:49 2018', 1, 8, 'delete'),
(10, 'Thu Jul 12 20:49:00 2018', 1, 9, 'delete'),
(11, 'Thu Jul 12 20:51:45 2018', 1, 10, 'start'),
(12, 'Thu Jul 12 20:53:43 2018', 1, 10, 'stop'),
(13, 'Thu Jul 12 20:54:29 2018', 1, 10, 'start');

-- --------------------------------------------------------

--
-- 表的结构 `user_list`
--

CREATE TABLE `user_list` (
  `user_id` int(10) UNSIGNED NOT NULL,
  `user_name` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `password` varchar(20) COLLATE utf8mb4_unicode_ci NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- 转存表中的数据 `user_list`
--

INSERT INTO `user_list` (`user_id`, `user_name`, `password`) VALUES
(1, 'Griffin', '411823');

-- --------------------------------------------------------

--
-- 表的结构 `__diesel_schema_migrations`
--

CREATE TABLE `__diesel_schema_migrations` (
  `version` varchar(50) NOT NULL,
  `run_on` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

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
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `task_edition`
--
ALTER TABLE `task_edition`
  ADD PRIMARY KEY (`id`);

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
  ADD KEY `user_id` (`user_id`);

--
-- Indexes for table `user_list`
--
ALTER TABLE `user_list`
  ADD PRIMARY KEY (`user_id`);

--
-- Indexes for table `__diesel_schema_migrations`
--
ALTER TABLE `__diesel_schema_migrations`
  ADD PRIMARY KEY (`version`);

--
-- 在导出的表使用AUTO_INCREMENT
--

--
-- 使用表AUTO_INCREMENT `device_actions`
--
ALTER TABLE `device_actions`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=2;

--
-- 使用表AUTO_INCREMENT `device_events`
--
ALTER TABLE `device_events`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=2;

--
-- 使用表AUTO_INCREMENT `device_event_logs`
--
ALTER TABLE `device_event_logs`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `device_list`
--
ALTER TABLE `device_list`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=6;

--
-- 使用表AUTO_INCREMENT `device_properties`
--
ALTER TABLE `device_properties`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=7;

--
-- 使用表AUTO_INCREMENT `task_assoc_devs`
--
ALTER TABLE `task_assoc_devs`
  MODIFY `id` int(10) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=13;

--
-- 使用表AUTO_INCREMENT `task_edition`
--
ALTER TABLE `task_edition`
  MODIFY `id` int(15) UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=8;

--
-- 使用表AUTO_INCREMENT `task_list`
--
ALTER TABLE `task_list`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=11;

--
-- 使用表AUTO_INCREMENT `user_action_logs`
--
ALTER TABLE `user_action_logs`
  MODIFY `id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=14;

--
-- 使用表AUTO_INCREMENT `user_list`
--
ALTER TABLE `user_list`
  MODIFY `user_id` int(10) UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=2;

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
-- 限制表 `task_list`
--
ALTER TABLE `task_list`
  ADD CONSTRAINT `task_list_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `user_list` (`user_id`);

--
-- 限制表 `user_action_logs`
--
ALTER TABLE `user_action_logs`
  ADD CONSTRAINT `user_action_logs_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `user_list` (`user_id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
