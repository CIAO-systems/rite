//
//  Generated code. Do not modify.
//  source: ciao/accounts/account_version.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use accountVersionDescriptor instead')
const AccountVersion$json = {
  '1': 'AccountVersion',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'user_id', '3': 2, '4': 1, '5': 9, '10': 'userId'},
    {'1': 'address', '3': 3, '4': 1, '5': 11, '6': '.ciao.common.Address', '10': 'address'},
    {'1': 'date', '3': 4, '4': 1, '5': 11, '6': '.ciao.common.Timestamp', '10': 'date'},
    {'1': 'custom_entitlement_reset_date', '3': 5, '4': 1, '5': 11, '6': '.ciao.common.Timestamp', '10': 'customEntitlementResetDate'},
    {'1': 'custom_entitlement_reset_date_active', '3': 6, '4': 1, '5': 8, '10': 'customEntitlementResetDateActive'},
    {'1': 'holiday_calendar_id', '3': 7, '4': 1, '5': 9, '10': 'holidayCalendarId'},
    {'1': 'shift_schedule_id', '3': 8, '4': 1, '5': 9, '10': 'shiftScheduleId'},
    {'1': 'vacation_balance_id', '3': 9, '4': 1, '5': 9, '10': 'vacationBalanceId'},
    {'1': 'cost_centers', '3': 10, '4': 3, '5': 11, '6': '.ciao.accounts.AccountCostCenter', '10': 'costCenters'},
    {'1': 'vacation_entitlement_offset', '3': 16, '4': 1, '5': 1, '10': 'vacationEntitlementOffset'},
    {'1': 'weekly_working_hours', '3': 17, '4': 1, '5': 1, '10': 'weeklyWorkingHours'},
    {'1': 'yearly_vacation_entitlement', '3': 18, '4': 1, '5': 1, '10': 'yearlyVacationEntitlement'},
  ],
};

/// Descriptor for `AccountVersion`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountVersionDescriptor = $convert.base64Decode(
    'Cg5BY2NvdW50VmVyc2lvbhIOCgJpZBgBIAEoCVICaWQSFwoHdXNlcl9pZBgCIAEoCVIGdXNlck'
    'lkEi4KB2FkZHJlc3MYAyABKAsyFC5jaWFvLmNvbW1vbi5BZGRyZXNzUgdhZGRyZXNzEioKBGRh'
    'dGUYBCABKAsyFi5jaWFvLmNvbW1vbi5UaW1lc3RhbXBSBGRhdGUSWQodY3VzdG9tX2VudGl0bG'
    'VtZW50X3Jlc2V0X2RhdGUYBSABKAsyFi5jaWFvLmNvbW1vbi5UaW1lc3RhbXBSGmN1c3RvbUVu'
    'dGl0bGVtZW50UmVzZXREYXRlEk4KJGN1c3RvbV9lbnRpdGxlbWVudF9yZXNldF9kYXRlX2FjdG'
    'l2ZRgGIAEoCFIgY3VzdG9tRW50aXRsZW1lbnRSZXNldERhdGVBY3RpdmUSLgoTaG9saWRheV9j'
    'YWxlbmRhcl9pZBgHIAEoCVIRaG9saWRheUNhbGVuZGFySWQSKgoRc2hpZnRfc2NoZWR1bGVfaW'
    'QYCCABKAlSD3NoaWZ0U2NoZWR1bGVJZBIuChN2YWNhdGlvbl9iYWxhbmNlX2lkGAkgASgJUhF2'
    'YWNhdGlvbkJhbGFuY2VJZBJDCgxjb3N0X2NlbnRlcnMYCiADKAsyIC5jaWFvLmFjY291bnRzLk'
    'FjY291bnRDb3N0Q2VudGVyUgtjb3N0Q2VudGVycxI+Cht2YWNhdGlvbl9lbnRpdGxlbWVudF9v'
    'ZmZzZXQYECABKAFSGXZhY2F0aW9uRW50aXRsZW1lbnRPZmZzZXQSMAoUd2Vla2x5X3dvcmtpbm'
    'dfaG91cnMYESABKAFSEndlZWtseVdvcmtpbmdIb3VycxI+Cht5ZWFybHlfdmFjYXRpb25fZW50'
    'aXRsZW1lbnQYEiABKAFSGXllYXJseVZhY2F0aW9uRW50aXRsZW1lbnQ=');

@$core.Deprecated('Use accountCostCenterDescriptor instead')
const AccountCostCenter$json = {
  '1': 'AccountCostCenter',
  '2': [
    {'1': 'cost_center_id', '3': 1, '4': 1, '5': 9, '10': 'costCenterId'},
    {'1': 'percentage', '3': 2, '4': 1, '5': 5, '10': 'percentage'},
  ],
};

/// Descriptor for `AccountCostCenter`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountCostCenterDescriptor = $convert.base64Decode(
    'ChFBY2NvdW50Q29zdENlbnRlchIkCg5jb3N0X2NlbnRlcl9pZBgBIAEoCVIMY29zdENlbnRlck'
    'lkEh4KCnBlcmNlbnRhZ2UYAiABKAVSCnBlcmNlbnRhZ2U=');

